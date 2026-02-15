mod config;
mod basic;

use crate::config::db::db;
use crate::basic::array::array_list;
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    array_list();
    db().await?;
    Ok(())
    
}