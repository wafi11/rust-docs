use axum::{
    Extension,Router
};
use tracing::{info, Level};

mod config;
mod modules;
mod routes;
mod encryption;

use crate::config::db::create_pool;
use crate::routes::api::create_routes;
use crate::encryption::encryption::{all_encryption};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    all_encryption();

    // Create connection pool
    let pool = create_pool().await?;
    
    // Create app with routes
    let app = Router::new()
        .nest("/api", create_routes())
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
        .await.unwrap();
    
    info!("Server is running on http://0.0.0.0:5000");
    axum::serve(listener, app).await?;
    
    Ok(())
}