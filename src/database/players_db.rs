use rusqlite::{Connection, Result};
use super::database_structs::{User};

pub fn get_players_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT username, color FROM users WHERE club_id = ?1")?;
    let user_iter = stmt.query_map([club_id], |row| {
        Ok(User {
            username: row.get(0)?,
            color: row.get(1)?,
        })
    })?;
    let mut users = Vec::new();
    for user in user_iter {
        users.push(user?);
    }
    Ok(users)
}