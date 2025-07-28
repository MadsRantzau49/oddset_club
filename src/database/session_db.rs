use rusqlite::params;

pub fn create_session_db(conn: &rusqlite::Connection, session_id: &String, club_id: i64) -> Result<String, rusqlite::Error> {
    conn.execute(
        "INSERT INTO sessions (session_id, club_id) VALUES (?1, ?2)",
        params![session_id, club_id],
    )?;

    Ok(session_id.to_string())
}


pub fn get_club_id_from_session_db(conn: &rusqlite::Connection, session_id: &str) -> Result<i64, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT club_id FROM sessions WHERE session_id = ?1")?;
    let club_id: i64 = stmt.query_row([session_id], |row| row.get(0))?;
    Ok(club_id)
}

pub fn terminate_session_db(conn: &rusqlite::Connection, session_id: &str, club_id: i64) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM sessions WHERE session_id = ?1 AND club_id = ?2",
        params![session_id, club_id],
    )?;
    Ok(())
}
