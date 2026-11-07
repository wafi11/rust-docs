use std::sync::Arc;

use axum::body::Body;
use axum::extract::multipart::{Multipart, MultipartError};
use axum::extract::{Path, State};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::Json;
use s3::error::S3Error;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::storage::{display_name, Storage, StoredFile};

const INDEX_HTML: &str = include_str!("index.html");

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Storage>,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Multipart(MultipartError),
    Io(std::io::Error),
    Storage(S3Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Multipart(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            Self::Io(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            Self::Storage(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<MultipartError> for AppError {
    fn from(value: MultipartError) -> Self {
        Self::Multipart(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<S3Error> for AppError {
    fn from(value: S3Error) -> Self {
        Self::Storage(value)
    }
}

pub async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

pub async fn list_files(
    State(state): State<AppState>,
) -> Result<Json<Vec<StoredFile>>, AppError> {
    let files = state.storage.list().await?;
    Ok(Json(files))
}

pub async fn upload(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    while let Some(mut field) = multipart.next_field().await? {
        if field.name() != Some("file") {
            continue;
        }

        let original = field
            .file_name()
            .map(str::to_string)
            .unwrap_or_else(|| "file".to_string());
        let key = format!("{}_{}", Uuid::new_v4(), sanitize_filename(&original));
        let temp_path =
            std::env::temp_dir().join(format!("mini-transfer-{}", Uuid::new_v4()));
        let mut temp_file = tokio::fs::File::create(&temp_path).await?;

        while let Some(chunk) = field.chunk().await? {
            temp_file.write_all(&chunk).await?;
        }
        temp_file.flush().await?;
        drop(temp_file);

        let mut reader = tokio::fs::File::open(&temp_path).await?;
        let uploaded = state.storage.upload(&key, &mut reader).await;
        drop(reader);
        tokio::fs::remove_file(&temp_path).await.ok();
        let size = uploaded?;

        return Ok((
            StatusCode::CREATED,
            Json(json!({ "key": key, "size": size })),
        ));
    }

    Err(AppError::BadRequest(
        "field 'file' tidak ditemukan".to_string(),
    ))
}

pub async fn download(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Response, AppError> {
    let stream = state.storage.download(&key).await?;
    let name = display_name(&key);
    let mime = mime_guess::from_path(&name).first_or_octet_stream();

    let mut response = Response::new(Body::from_stream(stream.bytes));
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(mime.as_ref())
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    if let Ok(value) = HeaderValue::from_str(&format!("attachment; filename=\"{name}\"")) {
        response
            .headers_mut()
            .insert(header::CONTENT_DISPOSITION, value);
    }

    Ok(response)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, AppError> {
    state.storage.delete(&key).await?;
    Ok(StatusCode::NO_CONTENT)
}

fn sanitize_filename(name: &str) -> String {
    let mut result: String = name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();

    if result.is_empty() {
        result.push_str("file");
    }
    result.truncate(120);
    result
}
