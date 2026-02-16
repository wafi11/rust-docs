// // Create connection pool
// let pool = create_pool().await?;
//
// // Create app with routes
// let app = Router::new()
// .nest("/api", create_routes())
// .layer(Extension(pool));
//
// let listener = tokio::net::TcpListener::bind("0.0.0.0:5000")
// .await?;
//
// info!("Server is running on http://0.0.0.0:5000");
// axum::serve(listener, app).await?;
//
// Ok(())