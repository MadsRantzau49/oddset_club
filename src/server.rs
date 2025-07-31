use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

pub mod router;

pub fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Listening on http://{}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let mut temp_buffer = [0; 512];

    // Step 1: Read until headers end (\r\n\r\n)
    loop {
        match stream.read(&mut temp_buffer) {
            Ok(0) => {
                // Connection closed before full request
                eprintln!("Connection closed unexpectedly");
                return;
            }
            Ok(n) => {
                buffer.extend_from_slice(&temp_buffer[..n]);
                if buffer.windows(4).any(|w| w == b"\r\n\r\n") {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                return;
            }
        }
    }

    // Convert buffer to string to parse headers
    let request_str = String::from_utf8_lossy(&buffer);
    // Split headers and body by first \r\n\r\n
    let parts: Vec<&str> = request_str.splitn(2, "\r\n\r\n").collect();
    let headers_str = parts[0];
    let mut body_bytes = if parts.len() > 1 { parts[1].as_bytes().to_vec() } else { Vec::new() };

    // Step 2: Parse Content-Length
    let content_length = headers_str
        .lines()
        .find_map(|line| {
            if line.to_lowercase().starts_with("content-length:") {
                line.split(':')
                    .nth(1)
                    .map(|v| v.trim().parse::<usize>().unwrap_or(0))
            } else {
                None
            }
        })
        .unwrap_or(0);

    // Step 3: Read the rest of the body if not fully read yet
    while body_bytes.len() < content_length {
        match stream.read(&mut temp_buffer) {
            Ok(0) => {
                eprintln!("Connection closed before full body read");
                return;
            }
            Ok(n) => {
                body_bytes.extend_from_slice(&temp_buffer[..n]);
            }
            Err(e) => {
                eprintln!("Error reading body: {}", e);
                return;
            }
        }
    }

    // Now you have full headers + full body in `headers_str` and `body_bytes`
    // You can reconstruct full request if needed
    let full_request = format!("{}\r\n\r\n{}", headers_str, String::from_utf8_lossy(&body_bytes));

    let response = router::route_request(&full_request);

    match response {
        ResponseBody::Text(text) => {
            let _ = stream.write_all(text.as_bytes());
        }
        ResponseBody::Binary(bytes) => {
            let _ = stream.write_all(&bytes);
        }
    }

    let _ = stream.flush();
}

pub enum ResponseBody {
    Text(String),     
    Binary(Vec<u8>), 
}
