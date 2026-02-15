mod config;
mod basic;
mod resp;
use crate::config::db::db;
use crate::resp::types::RootResponse;
use axum::{
    Extension, Json, Router, http::StatusCode, 
    routing::get,
    
};
use chrono_tz::Asia::Jakarta;
use sqlx::types::chrono::{Utc};
use tracing::{info, Level};

async fn get_root() -> Result<Json<RootResponse>, StatusCode>{
    let jakarta_time = Utc::now().with_timezone(&Jakarta);

    let respons = RootResponse{
        timestamp: jakarta_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        message: "Web Api Version 0.1",
        status:200,
    };

    Ok(Json(respons))
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    db().await?;
    
    let app = Router::new()
    .route("/", get(get_root))
        .layer(Extension(db));

 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await.unwrap();
    
    
    Ok(())
}
