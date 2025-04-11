use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use r2d2::Pool;

pub fn init_db() -> Result<Pool<SqliteConnectionManager>> {
    init_db_with_path("queue.db")
}

pub fn init_db_with_path(db_path: &str) -> Result<Pool<SqliteConnectionManager>> {
    let manager = SqliteConnectionManager::file(db_path);
    let pool = Pool::new(manager).expect("Failed to create pool");

    let conn = pool.get().expect("Failed to get connection from pool");
    conn.execute_batch(
        "
        DROP TABLE IF EXISTS messages;
        CREATE TABLE messages (
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
