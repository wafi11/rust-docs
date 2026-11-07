mod config;
mod storage;
mod web;

use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::routing::get;
use axum::Router;

use crate::storage::Storage;
use crate::web::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let config = config::Config::from_env().map_err(|error| std::io::Error::other(error))?;
    let storage = Storage::new(&config).await?;
    let state = AppState {
        storage: Arc::new(storage),
    };

    let app = Router::new()
        .route("/", get(web::index))
        .route("/files", get(web::list_files).post(web::upload))
        .route("/files/{key}", get(web::download).delete(web::delete))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    println!("Mini Transfer jalan di http://{}", config.bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
