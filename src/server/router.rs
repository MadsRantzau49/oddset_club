mod login;
mod error;
use std::collections::HashMap;

pub fn route_request(request: &str) -> String {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    let form_data = parse_form_data(request);

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

/// Parses form fields from a multipart/form-data POST request.
/// Returns an empty map if no valid form data is found.
fn parse_form_data(request: &str) -> HashMap<String, String> {
    let mut form_data = HashMap::new();

    // Find the body (after headers)
    let body_start = match request.find("\r\n\r\n") {
        Some(pos) => pos + 4,
        None => return form_data, // no body found
    };
    let body = &request[body_start..];

    // Get the boundary from the Content-Type header
    let boundary_line = request
        .lines()
        .find(|line| line.to_lowercase().contains("content-type: multipart/form-data; boundary="));
    let boundary = match boundary_line {
        Some(line) => line
            .split("boundary=")
            .nth(1)
            .map(|s| s.trim())
            .unwrap_or(""),
        None => return form_data, // no boundary found
    };

    let boundary_full = format!("--{}", boundary);

    // Iterate over form parts
    for part in body.split(&boundary_full) {
        if part.contains("Content-Disposition") {
            // Get the name="..."
            let name_line = match part.lines().find(|l| l.contains("name=")) {
                Some(line) => line,
                None => continue,
            };
            let name_start = match name_line.find("name=\"") {
                Some(pos) => pos + 6,
                None => continue,
            };
            let name_end = match name_line[name_start..].find('"') {
                Some(pos) => name_start + pos,
                None => continue,
            };
            let name = &name_line[name_start..name_end];

            // Get the value after \r\n\r\n
            let value = part.split("\r\n\r\n").nth(1).unwrap_or("").trim();
            let value = value.trim_matches('"'); // remove extra quotes

            form_data.insert(name.to_string(), value.to_string());
        }
    }

    form_data
}
