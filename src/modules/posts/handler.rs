use axum::{
    Extension, 
    Json, 
    http::StatusCode,
    extract::Path,
};
use crate::config::db::DbPool;
use super::post::{Post, CreatePost};
use super::repository;

// GET /posts - Get all posts
pub async fn get_posts(
    Extension(pool): Extension<DbPool>, 
) -> Result<Json<Vec<Post>>, StatusCode> {
    
    // ✅ Call repository function
    let posts = repository::get_all_posts(&pool)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(posts))
}

// GET /posts/:id - Get single post
pub async fn get_post(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    
    let post = repository::get_post_by_id(&pool, id)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error: {:?}", e);
            StatusCode::NOT_FOUND
        })?;
    
    Ok(Json(post))
}

// POST /posts - Create new post
pub async fn create_post(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    
    let post = repository::create_post(&pool, payload.name, payload.description)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    Ok(Json(post))
}

// PUT /posts/:id - Update post
pub async fn update_post(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
    Json(payload): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode> {
    
    let post = repository::update_post(&pool, id, payload.name, payload.description)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error: {:?}", e);
            StatusCode::NOT_FOUND
        })?;
    
    Ok(Json(post))
}

// DELETE /posts/:id - Delete post
pub async fn delete_post(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    
    let rows_affected = repository::delete_post(&pool, id)
        .await
        .map_err(|e| {
            eprintln!("❌ Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    
    Ok(StatusCode::NO_CONTENT)
}