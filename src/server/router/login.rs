use std::fs;

pub fn login(username: &str, password: &str) -> String{
    let correct_username = "mads";
    let correct_password = "123";

    if username == correct_username && password == correct_password{
        return match fs::read_to_string("src/frontend/html/index.html") {
            Ok(body) => format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
                body
            ),
            Err(_) => "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\nError loading page".to_string(),
        };
    }
    "Wrong information".to_string()
}