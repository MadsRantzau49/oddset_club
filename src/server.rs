use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

mod router;

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
    let mut buffer = [0; 1024];
    if let Ok(bytes_read) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..bytes_read]);
        let response = router::route_request(&request);
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}
