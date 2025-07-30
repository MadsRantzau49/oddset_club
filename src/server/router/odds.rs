use tera::Context;
use crate::database::players_db::{get_players_from_club_id};
use crate::database::odds_db::{insert_odds_db, get_all_odds_data_from_club_id, insert_result_db, get_number_of_unresolved_odds_db, delete_odds_db};
use crate::database::club_db::{get_club_settings_from_id};
use crate::database::establish_connection;
use crate::server::router::render::{render_error, render_template};

pub fn render_add_odds(club_id: i64) -> String {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_insert_odds_context(club_id);
    render_template("add_odds.html", &context)
}

pub fn insert_odds(club_id: i64, user_id: &str, stake: f64, odds: f64, potential_win: f64, description: &str, is_volunteer_bet: bool, is_gain_freebet: bool) -> String {
    let conn = establish_connection().expect("Could not connect to DB");
    let mut context = get_insert_odds_context(club_id);
    match insert_odds_db(&conn, user_id, stake, odds, potential_win, description, 0 ,is_volunteer_bet, is_gain_freebet) {
        Ok(_) => {
            context.insert("message", "Odds added succesfully");
        }
        Err(_) => {
            context.insert("message", "Odds failed to be added");
        }
    }
    render_template("add_odds.html", &context)
}

pub fn render_insert_odds(club_id: i64) -> String {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_insert_result_context(club_id);
    render_template("insert_result.html", &context)
}

pub fn insert_result(club_id: i64, odds_id: &str, result: i64) -> String{
    let conn = establish_connection().expect("Failed to connect to db");
    let update = insert_result_db(&conn, club_id,odds_id,result);
    let mut context = get_insert_result_context(club_id);
    match update {
        Ok(_) => {
            context.insert("message", "Result added succesfully");
        }
        Err(_) => {
            context.insert("message", "Result failed to be added");
        }
    }
    render_template("insert_result.html", &context)
}

pub fn delete_odds(club_id: i64, odds_id: &str) -> String{
    let conn = establish_connection().expect("Failed to connect DB");
    let delete = delete_odds_db(&conn, club_id, odds_id);
    let mut context = get_insert_result_context(club_id);
    match delete {
        Ok(_) => {
            context.insert("message", "Odds deleted succesfully");
        }
        Err(_) => {
            context.insert("message", "Failed to delete odds");
        }
    }
    render_template("insert_result.html", &context)
}

pub fn get_number_of_unresolved_odds(club_id: i64) -> i32 {
    let conn = establish_connection().expect("Could not connect to DB");
    match get_number_of_unresolved_odds_db(&conn, club_id){
        Ok(number) => number,
        Err(_) => 0
    }
}



fn get_insert_result_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to db");
    let mut context: Context = Context::new();
        match get_all_odds_data_from_club_id(&conn, club_id) {
            Ok(all_odds) => { context.insert("all_odds",&all_odds); }
            Err(_) => {}
        }
        context.insert("number_of_unresolved_odds", &get_number_of_unresolved_odds(club_id));
    context
}


fn get_insert_odds_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to db");
    let mut context = Context::new();
        match get_players_from_club_id(&conn, club_id) {
            Ok(users) => { context.insert("users",&users); }
            Err(_) => {}
        }
        match get_club_settings_from_id(&conn, club_id){
            Ok(setting) => {
                context.insert("default_stake", &setting.default_stake);
            }
            Err(_) => {}
        }
    context
}
