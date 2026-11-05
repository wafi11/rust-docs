use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub desc: String,
}

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn create(&self, name: &str, desc: &str) -> Result<Chat, sqlx::Error>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Chat>, sqlx::Error>;
    async fn list(&self, limit: i64) -> Result<Vec<Chat>, sqlx::Error>;
}

#[derive(Clone)]
pub struct PgChatRepository {
    db: PgPool,
}

impl PgChatRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ChatRepository for PgChatRepository {
    async fn create(&self, name: &str, desc: &str) -> Result<Chat, sqlx::Error> {
        let (id, name, desc): (i64, String, String) = sqlx::query_as(
            "INSERT INTO chats (name, desc) VALUES ($1, $2) RETURNING id, name, desc",
        )
        .bind(name)
        .bind(desc)
        .fetch_one(&self.db)
        .await?;

        Ok(Chat { id, name, desc })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Chat>, sqlx::Error> {
        let row: Option<(i64, String, String)> =
            sqlx::query_as("SELECT id, name, desc FROM chats WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.db)
                .await?;

        Ok(row.map(|(id, name, desc)| Chat { id, name, desc }))
    }

    async fn list(&self, limit: i64) -> Result<Vec<Chat>, sqlx::Error> {
        let rows: Vec<(i64, String, String)> =
            sqlx::query_as("SELECT id, name, desc FROM chats ORDER BY id DESC LIMIT $1")
                .bind(limit)
                .fetch_all(&self.db)
                .await?;

        Ok(rows
            .into_iter()
            .map(|(id, name, desc)| Chat { id, name, desc })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[derive(Default)]
    struct InMemoryChatRepository {
        chats: Mutex<Vec<Chat>>,
    }

    #[async_trait]
    impl ChatRepository for InMemoryChatRepository {
        async fn create(&self, name: &str, desc: &str) -> Result<Chat, sqlx::Error> {
            let mut chats = self.chats.lock().unwrap();
            let chat = Chat {
                id: chats.len() as i64 + 1,
                name: name.to_string(),
                desc: desc.to_string(),
            };
            chats.push(chat.clone());
            Ok(chat)
        }

        async fn find_by_id(&self, id: i64) -> Result<Option<Chat>, sqlx::Error> {
            Ok(self
                .chats
                .lock()
                .unwrap()
                .iter()
                .find(|chat| chat.id == id)
                .cloned())
        }

        async fn list(&self, limit: i64) -> Result<Vec<Chat>, sqlx::Error> {
            Ok(self
                .chats
                .lock()
                .unwrap()
                .iter()
                .rev()
                .take(limit as usize)
                .cloned()
                .collect())
        }
    }

    #[tokio::test]
    async fn repository_implementation_can_be_swapped() {
        let repo: Box<dyn ChatRepository> = Box::new(InMemoryChatRepository::default());

        repo.create("wafi", "halo").await.unwrap();
        repo.create("bot", "hai").await.unwrap();

        let chats = repo.list(10).await.unwrap();
        assert_eq!(chats.len(), 2);
        assert_eq!(chats[0].name, "bot");
        assert_eq!(repo.find_by_id(1).await.unwrap().unwrap().name, "wafi");
    }
}
