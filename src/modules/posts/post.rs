use serde::{Deserialize, Serialize};
use sqlx::FromRow;  


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub name: String,
    pub description: String,
}