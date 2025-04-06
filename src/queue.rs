use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct MessageQueue {
    pool: Pool<SqliteConnectionManager>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: String,
    pub body: String,
    pub topic: String,
    pub created_at: i64,
}

impl MessageQueue {
    pub fn new(pool: Pool<SqliteConnectionManager>) -> Self {
        Self { pool }
    }
    pub fn enqueue(&self, topic: String, body: String) -> Result<Uuid>  {
        let id = Uuid::new_v4();
        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.pool.get().expect("Failed to get connection from pool").execute(
            "INSERT INTO messages (id, created_at, topic, body) VALUES (?1, ?2, ?3, ?4)",
            [&id.to_string(), &created_at.to_string(), &topic, &body],
        )?;
        Ok(id)
    }

    pub fn dequeue(&self, topic: String) -> Result<Message> {
        let message = self.pool.get().expect("failed to get connection from pool").query_row(
            "SELECT id, created_at, topic, body FROM messages WHERE topic = ?1 AND acknowledged = 0 ORDER BY created_at ASC LIMIT 1",
            [&topic],
            |row| {
                Ok(Message {
                    id: row.get(0)?,
                    created_at: row.get(1)?,
                    topic: row.get(2)?,
                    body: row.get(3)?,
                })
            },
        )?;
        
        Ok(message)
    }

    pub fn acknowledge(&self, id: String) -> Result<()> {
        println!("Acknowledging message with id {}", id);
        self.pool.get().expect("failed to get connection from pool").execute(
            "UPDATE messages SET acknowledged = 1 WHERE id = ?1",
            [&id],
        )?;
        println!("Acknowledged message with id {}", id);
        Ok(())
    }
}
