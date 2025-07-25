mod login;
mod error;
use std::collections::HashMap;
use urlencoding::decode;

pub fn route_request(request: &str) -> String {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    let form_data = parse_urlencoded_form_data(request);

    if method == "GET" {
        return match path{
            "/login" => login::home_page(),
            "/" => login::home_page(),
            _ => error::not_found(),
        }
    }else if method == "POST" {
        return match path {
            "/login" => {
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let password = form_data.get("password").map(String::as_str).unwrap_or("");
                login::login(username, password)
            },

            "/create_user" => {
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let password = form_data.get("password").map(String::as_str).unwrap_or("");
                login::create_club(username, password)
            }
            _ => error::not_found(),
        
        }


    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found".to_string()
    }
}

fn parse_urlencoded_form_data(request: &str) -> HashMap<String, String> {
    let mut form_data = HashMap::new();

    // Find the body (after headers)
    let body_start = match request.find("\r\n\r\n") {
        Some(pos) => pos + 4,
        None => return form_data,
    };
    let body = &request[body_start..];

    // Split key=value pairs
    for pair in body.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let key = decode(key).unwrap_or_default();
            let value = decode(value).unwrap_or_default();
            form_data.insert(key.to_string(), value.to_string());
        }
    }

    form_data
}