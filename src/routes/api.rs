use axum::{Json, Router, http::StatusCode, routing::get};
use axum::routing::{delete};
use chrono::Utc;
use chrono_tz::Asia::Jakarta;
use serde::Serialize;

use crate::modules::posts::handler::{create_post, delete_post, get_post, get_posts, update_post};

#[derive(Serialize)]
pub struct RootResponse {
    pub timestamp: String,
    pub message: &'static str,
    pub status: i16
}

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(get_root))
        .route("/posts", get(get_posts).post(create_post))
        .route("/posts/{id}", delete(delete_post).put(update_post).get(get_post))
}

async fn get_root() -> Result<Json<RootResponse>, StatusCode> {
    let jakarta_time = Utc::now().with_timezone(&Jakarta);

    let response = RootResponse {
        timestamp: jakarta_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        message: "Web Api Version 0.1",
        status: 200,
    };

    Ok(Json(response))
}