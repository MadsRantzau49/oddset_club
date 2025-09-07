use tera::Context;
use crate::database::players_db::{get_players_from_club_id};
use crate::database::odds_db::{insert_odds_db, get_all_odds_data_from_club_id, insert_result_db, get_number_of_unresolved_odds_db, delete_odds_db,insert_sheet_odds_db, delete_all_odds};
use crate::database::club_db::{get_club_settings_from_id};
use crate::database::establish_connection;
use crate::server::router::render::{get_html, render_error, render_template};
use crate::server::ResponseBody;

pub fn render_add_odds(club_id: i64) -> ResponseBody {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_insert_odds_context(club_id);
    render_template("add_odds.html", &context)
}

pub fn insert_odds(club_id: i64, user_id: &str, stake: f64, odds: f64, potential_win: f64, description: &str, is_volunteer_bet: bool, is_gain_freebet: bool, is_freebet: bool) -> ResponseBody {
    let conn = establish_connection().expect("Could not connect to DB");
    let mut context = get_insert_odds_context(club_id);
    match insert_odds_db(&conn, user_id, stake, odds, potential_win, description, 0 ,is_volunteer_bet, is_gain_freebet, is_freebet) {
        Ok(_) => {
            context.insert("message", "Odds added succesfully");
        }
        Err(_) => {
            context.insert("message", "Odds failed to be added");
        }
    }
    render_template("add_odds.html", &context)
}

pub fn render_insert_odds(club_id: i64) -> ResponseBody {
    if club_id == 0 {return render_error("Session dead");}
    let context = get_insert_result_context(club_id);
    render_template("insert_result.html", &context)
}

pub fn insert_result(club_id: i64, odds_id: &str, result: i64) -> ResponseBody{
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

pub fn delete_odds(club_id: i64, odds_id: &str) -> ResponseBody{
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

use calamine::{open_workbook_auto, Reader, DataType};

pub fn insert_odds_from_excel(club_id: i64) -> ResponseBody {
    let conn = establish_connection().expect("Cannot connect to DB");
    match delete_all_odds(&conn) {
        Ok(_) => {},
        Err(_) => {},
    }
    let mut workbook = match open_workbook_auto("data/sheet.xlsx") {
        Ok(wb) => wb,
        Err(e) => return render_error(format!("failed to open sheet: {}", e).as_str()),
    };

    let range = match workbook.worksheet_range("Odds Data") {
        Some(Ok(r)) => r,
        Some(Err(e)) => return render_error(&format!("Failed to parse sheet: {}", e)),
        None => return render_error("Sheet 'Odds Data' not found"),
    };


    for row in range.rows().skip(1) {
        let get_f64 = |cell: &DataType| cell.get_float().unwrap_or(0.0);
        let get_string = |cell: &DataType| cell.get_string().unwrap_or("").to_string();

        let users = match get_players_from_club_id(&conn, club_id) {
            Ok(users) => users,
            Err(_) => continue
        };

        let mut user_id: i64 = 0; 
        
        for user in users {
            if user.username == get_string(&row[1]) && user.username != "Casino".to_string(){
                user_id = user.id;
            }
        }
        if user_id == 0 {continue};

        let mut stake: f64 = get_f64(&row[3]);
        let odds: f64 = get_f64(&row[4]);
        let potential_win: f64 = odds * stake;
        let description: String = get_string(&row[2]);
        let result_str = get_string(&row[5]);
        
        let result: i64;
        let mut is_volunteer_bet = false;

        if result_str == "Vundet" {
            result = 1;
        } else if result_str == "Tabt" {
            result = 2;
        } else if result_str == "Ugyldig" {
            is_volunteer_bet = true;
            result = 2;
        } else {
            result = 0;
        }


        let is_gain_freebet: bool = get_string(&row[2]).contains("freebet");

        let created_at: String = "2025-03-03 20:00:00".to_string();

        if is_volunteer_bet {
            stake = 0.0;
        }

        let is_freebet = user_id == 6;
        
        match insert_sheet_odds_db(&conn, user_id, stake, odds, potential_win, description, result, is_volunteer_bet, is_gain_freebet, created_at, is_freebet){
            Ok(_) => {}
            Err(e) => {println!("{}\n",e)}
        }
    }
    get_html("dashboard.html", club_id)
}