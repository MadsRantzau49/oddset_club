use tera::Context;
use crate::database::players_db::{get_players_from_club_id};
use crate::database::debt_db::{get_debt_from_club_id, insert_debt_db, delete_debt_db, mark_debt_paid_db, get_number_of_unpaid_debts_db };
use crate::database::establish_connection;
use crate::server::router::render::{render_error, render_template};

pub fn render_debt(club_id: i64) -> String {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_debt_context(club_id);
    render_template("debt.html", &context)
}

pub fn insert_debt(user_id: &str, amount: f64, description: &str, club_id: i64) -> String {
    let conn = establish_connection().expect("Could not connect to DB");
    let debt_db = insert_debt_db(&conn, user_id, amount, description);
    let mut context = get_debt_context(club_id);
    match debt_db{
        Ok(_) => {
            context.insert("message", "Debt succesfully added");
        }
        Err(_) => {
            context.insert("message", "Debt failed to be added");
        }
    }
    render_template("debt.html", &context)
}

pub fn delete_debt(debt_id: &str, club_id: i64) -> String {
    let conn = establish_connection().expect("Could not connect to DB");
    let debt_db = delete_debt_db(&conn, debt_id, club_id);
    let mut context = get_debt_context(club_id);
    match debt_db{
        Ok(_) => {
            context.insert("message", "Debt deleted succesfully");
        }
        Err(_) => {
            context.insert("message", "Debt failed to be deleted");
        }
    }
    render_template("debt.html", &context)
}

pub fn mark_debt_paid(debt_id: &str, club_id: i64) -> String {
    let conn = establish_connection().expect("Could not connect to DB");
    let debt_db = mark_debt_paid_db(&conn, debt_id, club_id);
    let mut context = get_debt_context(club_id);
    match debt_db{
        Ok(_) => {
            context.insert("message", "Debt succesfully marked as paid");
        }
        Err(_) => {
            context.insert("message", "Debt failed to be marked as paid");
        }
    }
    render_template("debt.html", &context)
}

pub fn get_number_of_unpaid_debts(club_id: i64) -> i32 {
    let conn = establish_connection().expect("Could not connect to DB");
    match get_number_of_unpaid_debts_db(&conn, club_id){
        Ok(number) => number,
        Err(_) => 0
    }
}

fn get_debt_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to db");
    let mut context = Context::new();
        match get_players_from_club_id(&conn, club_id) {
            Ok(users) => { context.insert("users",&users); }
            Err(_) => {}
        }
        match get_debt_from_club_id(&conn, club_id){
            Ok(debt) => { context.insert("debts", &debt);}
            Err(_) => {}
        }
        context.insert("number_of_unpaid_debts", &get_number_of_unpaid_debts(club_id));
    context
}