use tera::Context;
use crate::database::players_db::{get_players_from_club_id};
use crate::database::money_insertion_db::{get_money_insertion_from_club_id,insert_money_insertion_db, delete_money_insertion_db};
use crate::database::establish_connection;
use crate::server::router::render::{render_error, render_template};
use crate::server::ResponseBody;

pub fn render_insert_money(club_id: i64) -> ResponseBody {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_insert_money_context(club_id);
    render_template("insert_money.html", &context)
}

pub fn insert_money_insertion(user_id: &str, amount: f64, club_id: i64, is_volunteer_bet: bool) -> ResponseBody {
    let conn = establish_connection().expect("Could not connect to DB");
    let insertion_db = insert_money_insertion_db(&conn, user_id, amount, is_volunteer_bet);
    let mut context = get_insert_money_context(club_id);
    match insertion_db{
        Ok(_) => {
            context.insert("message", "Transaction succesfully added");
        }
        Err(e) => {
            println!("{e}");
            context.insert("message", "Transaction failed to be added");
        }
    }
    render_template("insert_money.html", &context)
}

pub fn delete_insertion(insertion_id: &str, club_id: i64) -> ResponseBody {
      let conn = establish_connection().expect("Could not connect to DB");
    let insertion_db = delete_money_insertion_db(&conn, insertion_id, club_id);
    let mut context = get_insert_money_context(club_id);
    match insertion_db{
        Ok(_) => {
            context.insert("message", "Transaction deleted succesfully");
        }
        Err(_) => {
            context.insert("message", "Transaction failed to be deleted");
        }
    }
    render_template("insert_money.html", &context)
}

fn get_insert_money_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to db");
    let mut context = Context::new();
        match get_players_from_club_id(&conn, club_id) {
            Ok(users) => { context.insert("users",&users); }
            Err(_) => {}
        }
        match get_money_insertion_from_club_id(&conn, club_id){
            Ok(money_insertions) => { context.insert("insertions", &money_insertions); }
            Err(_) => {}
        }
    context
}