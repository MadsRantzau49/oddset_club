use rusqlite::{Connection, Result};

pub mod users_db;

pub fn establish_connection() -> Result<Connection> {
    Connection::open("data/database.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clubs (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

