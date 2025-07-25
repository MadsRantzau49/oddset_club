use rusqlite::{Connection, Result};

pub fn add_club(conn: &Connection, username: &str, password: &str) -> Result<bool> {
    conn.execute(
        "INSERT INTO clubs (username, password) VALUES (?1, ?2)",
        [username, password],
    )?;
    Ok(true)
}

pub fn verify_club(conn: &Connection, username: &str, password: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM clubs WHERE username = ?1 AND password = ?2")?;
    let count: i64 = stmt.query_row([username, password], |row| row.get(0))?;
    Ok(count == 1)
}