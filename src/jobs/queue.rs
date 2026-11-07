use std::sync::Arc;

use bullmq::{
    worker::{CancellationToken, ProcessorFn},
    Job, Queue, QueueOptions, Worker,
};
use serde::Serialize;

use crate::config::redis::redis_connection;

#[derive(Debug, Clone, Serialize)]
pub struct SaldoAwalStock<'a> {
    pub barang: &'a str,
    pub qty: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct State<'a> {
    pub gudang: String,
    pub data: Vec<SaldoAwalStock<'a>>,
}

#[derive(Debug, Clone)]
pub struct QueueData<'a> {
    pub topic: &'a str,
    pub data: Vec<State<'a>>,
}

#[derive(Debug, Default)]
pub struct QueueService<'a> {
    pub queues: Vec<QueueData<'a>>,
}

impl<'a> QueueService<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_queue(&mut self, topic: &'a str, data: Vec<State<'a>>) {
        self.queues.push(QueueData { topic, data });
    }

    pub fn choice_queue(&self, topic: &'a str) {
        match topic {
            "SALDO_AWAL_STOK" => println!("{:?}", topic),
            "STOK_OPNAME" => println!("{:?}", topic),
            "INVOICE_PEMBELIAN" => println!("{:?}", topic),
            _ => println!("unknown"),
        }
    }

    fn make_prefix(topic: &str) -> String {
        format!("{{{}}}", topic)
    }

    fn first(&self) -> Result<&QueueData<'a>, bullmq::Error> {
        self.queues
            .first()
            .ok_or_else(|| bullmq::Error::InvalidConfig("queue belum dibuat".to_string()))
    }

    pub async fn queue(&self) -> Result<Queue, bullmq::Error> {
        let data = self.first()?;

        let queue = Queue::with_options(
            data.topic,
            QueueOptions {
                connection: redis_connection(),
                prefix: Self::make_prefix(data.topic),
                ..Default::default()
            },
        )
        .await?;

        queue.add(data.topic, &data.data).await?;

        Ok(queue)
    }

    pub async fn worker(&self) -> Result<Worker, bullmq::Error> {
        let data = self.first()?;

        let processor: ProcessorFn = Arc::new(|job: Job, _token: CancellationToken| {
            Box::pin(async move {
                println!("data : {:?}", job.data());
                Ok(serde_json::json!(null))
            })
        });

        Worker::with_options(
            data.topic,
            processor,
            bullmq::WorkerOptions {
                connection: redis_connection(),
                prefix: Self::make_prefix(data.topic),
                ..Default::default()
            },
        )
        .await
    }
}
