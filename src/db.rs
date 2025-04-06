use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("queue.db")?;
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS messages (
            id TEXT PRIMARY KEY,
            body TEXT NOT NULL,
            acknowledged INTEGER DEFAULT 0
        );
        ",
    )?;
    Ok(conn)
}
