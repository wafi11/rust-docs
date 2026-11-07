// use redis::{Client, RedisError};
// use sqlx::{PgPool, postgres::PgPoolOptions};

// pub struct RedisConfig {
//     pub url: String
// }

// pub struct DBConfig {
//     pub url: String,
// }

// pub fn redis_client(cfg: &RedisConfig) -> Result<Client, RedisError> {
//     redis::Client::open(cfg.url.clone())
// }

// pub async fn get_connection_db(cfg: &DBConfig) -> Result<PgPool, sqlx::Error> {
//     let db = PgPoolOptions::new()
//         .max_connections(2)
//         .connect(&cfg.url)
//         .await?;

//     println!("successfully connect");
//     Ok(db)
// }
