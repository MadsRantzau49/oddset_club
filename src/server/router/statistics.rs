use rusqlite::Connection;
use tera::{Context};

use crate::{database::{club_db::get_club_settings_from_id, establish_connection, money_insertion_db::get_money_insertion_from_club_id, odds_db::{self, get_all_odds_data_from_club_id, get_oldest_odds}}, server::router::{render::render_template}};
use chrono::{NaiveDateTime, Utc, Duration};
use crate::server::ResponseBody;


pub fn renderer_statistics(club_id: i64) -> ResponseBody{
    let context = get_statistics_context(club_id,"2021-01-05", "2026-01-01");
    render_template("statistics.html", &context)    
}

fn get_statistics_context(club_id: i64, start_date: &str, end_date: &str) -> Context {
    let conn = establish_connection().expect("Failed to connect to DB");
    let mut context = Context::new();

    // Summary stats
    context.insert("total_balance", &get_total_balance(club_id));
    context.insert("total_deposit", &get_total_deposit(club_id));
    context.insert("total_oddset_balance", &get_total_odds_balance(&conn, club_id));
    context.insert("total_money", &get_total_money(&conn, club_id));
    context.insert("expected_travel_date", &expected_travel_date(&conn, club_id));

    // For chart.js
    context.insert("bet_labels", &vec!["July 1", "July 2", "July 3"]);
    context.insert("bet_profits", &vec![50.0, -30.0, 120.0]);
    context.insert("bet_colors", &vec!["#4caf50", "#dc3545", "#4caf50"]);

    context
}

fn get_total_balance(club_id: i64) -> f64 {
    let conn = establish_connection().expect("Cannot connect to DB");
    let mut sum: f64 = get_total_odds_balance(&conn, club_id);

    sum += get_total_insertions(&conn, club_id);

    sum
}

fn get_total_deposit(club_id: i64) -> f64 {
    let conn = establish_connection().expect("Cannot connect to DB");
    let mut sum: f64 = match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut deposit = 0.0;

            for odds in oddss{
                //Ignore pending
                if odds.result != 0{
                    deposit += odds.stake;
                }
            }
            deposit
        }
        Err(_) => 0.0
    };

    sum += get_total_insertions(&conn, club_id);

    sum
}


fn get_total_insertions(conn: &Connection, club_id: i64) -> f64{
    match get_money_insertion_from_club_id(&conn, club_id){
        Ok(insertions) => {
            let mut temp_sum: f64 = 0.0;
            for insertion in insertions {
                temp_sum += insertion.amount;
            }
            temp_sum
        }
        Err(_) => 0.0
    }   
}

fn get_total_odds_balance(conn: &Connection, club_id: i64) -> f64{
    match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut stake = 0.0;
            let mut won = 0.0;

            for odds in oddss{
                //Ignore pending odds
                if odds.result != 0{
                    stake+= odds.stake;
                    if odds.result == 1{
                        won += odds.potential_win;
                    }
                }
  
            }
            won - stake
        }
        Err(_) => 0.0
    }
}

pub fn get_total_money(conn: &Connection, club_id: i64) -> f64 {
    get_total_insertions(&conn, club_id) + get_total_odds_win(conn, club_id)
}

fn get_total_odds_win(conn: &Connection, club_id: i64) -> f64{
    match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut won = 0.0;
            for odds in oddss {
                if odds.result == 1 {
                    won += odds.potential_win;
                }
            }
            won
        }
        Err(_) => 0.0
    }
}


pub fn expected_travel_date(conn: &Connection, club_id: i64) -> String {
    match odds_db::insert_old_odds_db(conn) {
        Ok(_) => {},
        Err(e) => {println!("{}",e);}
    }
    let money_earned = get_total_money(conn, club_id);
    let money_goal = match get_club_settings_from_id(conn, club_id) {
        Ok(setting) => setting.money_goal,
        Err(_) => return "-".to_string(),
    };

    if money_earned <= 0.0 {
        return "-".to_string(); // No progress made
    }

    let start_date_str = match get_oldest_odds(conn, club_id) {
        Ok(date) => date,
        Err(_) => return "-".to_string(),
    };


    let start_date = match NaiveDateTime::parse_from_str(&start_date_str, "%Y-%m-%d %H:%M:%S") {
        Ok(d) => d,
        Err(_) => return "-".to_string(),
    };

    let now = Utc::now().naive_utc();
    let days = (now - start_date).num_days();

    if days <= 0 {
        return "-".to_string(); // Not enough time passed
    }

    let daily_rate = money_earned / days as f64;
    if daily_rate <= 0.0 {
        return "-".to_string(); // No positive earning rate
    }

    let remaining = money_goal - money_earned;
    let estimated_days_needed = (remaining / daily_rate).ceil() as i64;
    let projected_date = now + Duration::days(estimated_days_needed);

    projected_date.format("%Y-%m-%d").to_string()
}
