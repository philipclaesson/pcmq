use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use r2d2::Pool;

pub fn init_db() -> Result<Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file("queue.db");
    let pool = Pool::new(manager).expect("Failed to create pool");

    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            body TEXT NOT NULL,
            topic TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            acknowledged INTEGER DEFAULT 0
        );
        ",
    )?;
    Ok(pool)
}
