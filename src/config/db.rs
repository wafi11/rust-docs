use std::env;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn db() -> Result<(), sqlx::Error>{
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Create a connection pool
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await?;

    println!("Connected to the database!");
    let row: (String,) = sqlx::query_as("SELECT name FROM users LIMIT 1 OFFSET 0")
        .fetch_one(&pool)
        .await?; 

    println!("Query result: {}", row.0);

    _ = pool;

    Ok(())
}