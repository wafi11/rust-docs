use std::time::Duration;

use apalis::prelude::*;
use apalis_redis::{ConnectionManager, RedisStorage};

use crate::queue::ChatJob;

pub async fn run(
    storage: RedisStorage<ChatJob, ConnectionManager>,
) -> Result<(), BoxDynError> {
    async fn handle(
        job: ChatJob,
        storage: Data<RedisStorage<ChatJob, ConnectionManager>>,
    ) -> Result<(), BoxDynError> {
        println!("memproses job: {} - {}", job.name, job.desc);
        tokio::time::sleep(Duration::from_millis(500)).await;

        if !job.is_reply {
            let mut store = storage.0.clone();
            store
                .push(ChatJob {
                    name: "bot".to_string(),
                    desc: format!("Halo {}, pesanmu sudah masuk antrean: \"{}\"", job.name, job.desc),
                    is_reply: true,
                })
                .await?;

            
        }


        Ok(())
    }

    WorkerBuilder::new("chat_worker")
        .backend(storage.clone())
        .data(storage)
        .build(handle)
        .run()
        .await?;

    Ok(())
}
