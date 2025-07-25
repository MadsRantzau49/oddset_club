use std::fs;

use crate::database::establish_connection;
use crate::database::users_db::{verify_club,add_club};

pub fn login(username: &str, password: &str) -> String{
    let conn = establish_connection().expect("Failed to connect to DB");
    if verify_club(&conn, username, password).unwrap_or(false){
        return match fs::read_to_string("src/frontend/html/dashboard.html") {
            Ok(body) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                body
            ),
            Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nError loading page".to_string(),
        };
    }
    "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\nWrong information".to_string()
}

pub fn create_club(username: &str, password: &str) -> String{
    let conn = establish_connection().expect("Failed to connect to DB");
    if add_club(&conn, username, password).unwrap_or(false){
        return login(username,password);
    }
    "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\n\r\nFailed to add club".to_string()
}

pub fn home_page() -> String{
    return match fs::read_to_string("src/frontend/html/index.html") {
        Ok(body) => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            body
        ),
        Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nError loading page".to_string(),
    };
}