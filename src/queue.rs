use std::time::Duration;

use apalis_redis::{ConnectionManager, RedisStorage};
use serde::{Deserialize, Serialize};

use crate::config::{RedisConfig};

pub const QUEUE_NAME: &str = "chat";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatJob {
    pub name: String,
    pub desc: String,
    #[serde(default)]
    pub is_reply: bool,
}

pub struct Queue {
    pub storage: RedisStorage<ChatJob, ConnectionManager>,
    pub redis: ConnectionManager,
}

impl Queue {
    pub async fn new(config: &RedisConfig) -> Result<Self, apalis_redis::Error> {
        let conn = apalis_redis::connect(config.url.clone()).await?;
        let redis = conn.clone();
        let storage = RedisStorage::new(conn).with_config(
            apalis_redis::Config::default()
                .queue(QUEUE_NAME)
                .heartbeat_interval(Duration::from_secs(1))
                .missed_heartbeats(10),
        );
        Ok(Self { storage, redis })
    }
}
