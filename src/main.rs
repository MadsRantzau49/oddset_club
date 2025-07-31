mod server;
mod database;

fn main() {
    let conn = database::establish_connection().expect("DB failed");
    database::init_db(&conn).expect("DB init failed");
    server::run("127.0.0.1:8000").expect("Server failed");
}