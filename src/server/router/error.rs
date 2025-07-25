use std::fs;

pub fn not_found() -> String{
    return match fs::read_to_string("src/frontend/html/404.html") {
        Ok(body) => format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
            body
        ),
        Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nError loading page".to_string(),
    };
}