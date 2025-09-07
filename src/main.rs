mod server;
mod database;
use database::sql;

fn main() {
    let conn = database::establish_connection().expect("DB failed");
    database::init_db(&conn).expect("DB init failed");
    //Run SQL Code:
    match sql::run_sql(&conn){
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
        }
    }


    server::run("0.0.0.0:8000").expect("Server failed");

}