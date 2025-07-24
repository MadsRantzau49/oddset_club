mod login;

pub fn route_request(request: &str) -> String {
    let mut parts = request.split_whitespace();

    let method = parts.next().unwrap_or("GET").to_string();
    // let path = parts.next().unwrap_or("/").to_string();

    if method == "GET" {
        login::login("mads","123")
    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found".to_string()
    }
}
