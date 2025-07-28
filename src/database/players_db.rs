use rusqlite::{Connection, Result, params};
use super::database_structs::{User};

pub fn get_players_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT id, username, color FROM users WHERE club_id = ?1")?;
    let user_iter = stmt.query_map([club_id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            color: row.get(2)?,
        })
    })?;
    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    Ok(users)
}

pub fn add_user_db(conn: &Connection, club_id: i64, username: &str, color: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, color, club_id) VALUES (?1, ?2, ?3)",
        params![username, color, club_id],
    )?;
    Ok(())
}

pub fn delete_user_db(conn: &Connection, club_id: i64, user_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM users WHERE club_id = ?1 AND id = ?2", 
        params![club_id, user_id],
    )?;
    Ok(())
}

pub fn edit_user_db(conn: &Connection, club_id: i64, username: &str, color: &str, user_id: &str) -> Result<()> {
    conn.execute(
        "UPDATE users SET username = ?1, color = ?2 WHERE club_id = ?3 AND id = ?4",
        params![username, color, club_id, user_id],
    )?;
    Ok(())
}
