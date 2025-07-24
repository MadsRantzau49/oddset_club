mod server;

fn main() {
    println!("Starting server...");
    server::run("127.0.0.1:8080").expect("Server failed");
}
