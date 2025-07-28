use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use uuid::Uuid;
use crate::database::session_db::{create_session_db, get_club_id_from_session_db, terminate_session_db};
use crate::database::establish_connection;
use super::render::{get_html};

lazy_static! {
    static ref SESSION_STORE: Mutex<HashMap<String, i64>> = Mutex::new(HashMap::new());
}

pub fn create_session(club_id: i64) -> String {
    let conn: rusqlite::Connection = establish_connection().expect("Failed to connect to DB");
    let session_id = Uuid::new_v4().to_string();
    create_session_db(&conn, &session_id, club_id).unwrap_or("".to_string())
}

pub fn get_club_id_from_session(session_id: &String) -> Option<i64> {
    let conn: rusqlite::Connection = establish_connection().expect("Failed to connect to DB");
    match get_club_id_from_session_db(&conn, &session_id){
        Ok(club_id) => Some(club_id),
        Err(_) => None,
    }
}

pub fn terminate_session(club_id: i64, session_id: &String) -> String {
    let conn: rusqlite::Connection = establish_connection().expect("Failed to connect to DB");
    match terminate_session_db(&conn, session_id, club_id) {
        Ok(_) => get_html("index.html", 0),
        Err(_) => get_html("index.html", 0),
    }
}