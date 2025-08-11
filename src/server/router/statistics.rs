use rusqlite::Connection;
use tera::{Context};

use crate::database::establish_connection;
use crate::database::club_db::{get_club_settings_from_id};
use crate::database::money_insertion_db::{get_money_insertion_from_club_id};
use crate::database::odds_db::{get_all_odds_data_from_club_id, get_oldest_odds, get_user_odds_data};
use crate::server::router::{render::render_template};
use chrono::{NaiveDateTime, Utc, Duration};
use crate::server::ResponseBody;
use crate::database::players_db::{get_players_from_club_id};
use crate::database::money_insertion_db::{get_user_money_insertions};
use crate::database::database_structs::{UserStatistic};
use std::collections::HashMap;

pub fn renderer_statistics(club_id: i64) -> ResponseBody{
    let context = get_statistics_context(club_id);
    render_template("statistics.html", &context)    
}

fn get_statistics_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to DB");
    let mut context = Context::new();

    // Summary stats
    context.insert("total_balance", &get_total_balance(club_id));
    context.insert("total_deposit", &get_total_deposit(club_id));
    context.insert("total_oddset_balance", &get_total_odds_balance(&conn, club_id));
    context.insert("total_money", &get_total_money(&conn, club_id));
    context.insert("expected_travel_date", &expected_travel_date(&conn, club_id));

    context.insert("player_stats_list", &get_each_user_statistic(&conn, club_id));

    // For chart.js
    context.insert("bet_labels", &vec!["July 1", "July 2", "July 3"]);
    context.insert("bet_profits", &vec![50.0, -30.0, 120.0]);
    context.insert("bet_colors", &vec!["#4caf50", "#dc3545", "#4caf50"]);

    context
}

fn get_total_balance(club_id: i64) -> i64 {
    let conn = establish_connection().expect("Cannot connect to DB");
    let mut sum: i64 = get_total_odds_balance(&conn, club_id);

    sum += get_total_insertions_cib(&conn, club_id);

    sum
}

fn get_total_deposit(club_id: i64) -> i64 {
    let conn = establish_connection().expect("Cannot connect to DB");
    let mut sum: i64 = match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut deposit = 0.0;

            for odds in oddss{
                //Ignore pending
                if odds.result != 0 && !odds.is_freebet{
                    deposit += odds.stake;
                }
            }
            deposit.round() as i64
        }
        Err(_) => 0
    };

    sum += get_total_insertions_without_cib(&conn, club_id) ;

    sum
}
//Count in balance = CIB
fn get_total_insertions_without_cib(conn: &Connection, club_id: i64) -> i64{
    match get_money_insertion_from_club_id(&conn, club_id){
        Ok(insertions) => {
            let mut temp_sum: f64 = 0.0;
            for insertion in insertions {
                if !insertion.is_valid_balance{
                    temp_sum += insertion.amount;
                }
            }
            temp_sum.round() as i64
        }
        Err(_) => 0
    }   
}

//Count in balance = CIB
fn get_total_insertions_cib(conn: &Connection, club_id: i64) -> i64{
    match get_money_insertion_from_club_id(&conn, club_id){
        Ok(insertions) => {
            let mut temp_sum: f64 = 0.0;
            for insertion in insertions {
                if insertion.is_valid_balance{
                    temp_sum += insertion.amount;
                }
            }
            temp_sum.round() as i64
        }
        Err(_) => 0
    }   
}

fn get_total_odds_balance(conn: &Connection, club_id: i64) -> i64{
    match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut stake = 0.0;
            let mut won: f64 = 0.0;

            for odds in oddss{
                //Ignore pending odds
                if odds.result != 0{
                    if !odds.is_freebet {
                        stake+= odds.stake;
                    }
                    if odds.result == 1{
                        won += odds.potential_win;
                    }
                }
  
            }
            (won - stake).round() as i64
        }
        Err(_) => 0
    }
}

pub fn get_total_money(conn: &Connection, club_id: i64) -> i64 {
    get_total_insertions_cib(&conn, club_id) + get_total_insertions_without_cib(conn, club_id) + get_total_odds_win(conn, club_id)
}

fn get_total_odds_win(conn: &Connection, club_id: i64) -> i64{
    match get_all_odds_data_from_club_id(&conn, club_id){
        Ok(oddss) => {
            let mut won = 0.0;
            for odds in oddss {
                if odds.result == 1 {
                    won += odds.potential_win;
                }
            }
            won.round() as i64
        }
        Err(_) => 0
    }
}


pub fn expected_travel_date(conn: &Connection, club_id: i64) -> String {
    let money_earned = get_total_money(conn, club_id) as f64;
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

fn get_each_user_statistic(conn: &Connection, club_id: i64) -> Vec<UserStatistic> {
    let mut user_statistics_vector = Vec::new();
    let mut user_statistics = HashMap::new();
    match get_players_from_club_id(&conn, club_id) {
        Ok(users) => { 
            for user in users{
                user_statistics.insert(user.id, UserStatistic{
                    username: user.username,
                    color: user.color,
                    total_balance: get_user_balance(&conn, user.id),
                    total_won: get_user_total_won(&conn, user.id),
                    winrate: get_user_win_rate(&conn, user.id),
                    total_deposit: get_user_deposit(&conn, user.id),
                    amount_of_freebets: get_user_amount_of_freebets(&conn, user.id),
                });
            }
        }
        Err(_) => {}
    }    
    for (_key,val) in user_statistics{
        user_statistics_vector.push(val);
    }
    user_statistics_vector.sort_by(|a, b| b.username.cmp(&a.username));

    user_statistics_vector
}

fn get_user_balance(conn: &Connection, user_id: i64) -> i64 {
    let mut sum = 0.0;
    match get_user_odds_data(&conn, user_id){
        Ok(oddss) => {
            for odds in oddss {
                if odds.result == 1 {
                    sum += odds.potential_win - odds.stake;
                } else if odds.result == 2 {
                    if !odds.is_volunteer_bet {
                        sum -= odds.stake;
                    }
                }
            }
        }
        Err(e) => {println!("{e}");}
    }
    sum.round() as i64
}

fn get_user_amount_of_freebets(conn: &Connection, user_id: i64) -> i64 {
    let mut sum = 0;
    match get_user_odds_data(&conn, user_id){
        Ok(oddss) => {
            for odds in oddss {
                if odds.is_gain_freebet {
                    sum += 1;
                } 
            }
        }
        Err(e) => {println!("{e}");}
    }
    sum
}

fn get_user_total_won(conn: &Connection, user_id: i64) -> i64 {
    let mut sum = 0.0;
    match get_user_odds_data(&conn, user_id){
        Ok(oddss) => {
            for odds in oddss {
                if odds.result == 1 {
                    sum += odds.potential_win;
                } 
            }
        }
        Err(e) => {println!("{e}");}
    }
    sum.round() as i64
}

fn get_user_win_rate(conn: &Connection, user_id: i64) -> f64 {
    match get_user_odds_data(&conn, user_id) {
        Ok(oddss) => {
            let len = oddss.len() as f64;
            if len == 0.0 {
                return 0.0;
            }
            let mut wins = 0.0;
            for odds in oddss {
                if odds.result == 1 {
                    wins += 1.0;
                }
            }
            return ((wins / len) * 100.0).round();
        }
        Err(_) => 0.0,
    }
}

fn get_user_deposit(conn: &Connection, user_id: i64) -> i64 {
    let mut sum = 0.0;
    match get_user_odds_data(&conn, user_id) {
        Ok(oddss) => {
            for odds in oddss{
                sum += odds.stake;
            }
        }
        Err(_) => {}
    }

    match get_user_money_insertions(&conn, user_id) {
        Ok(insertions) => {
            for insertion in insertions{
                sum += insertion;
            }
        }
        Err(_) => {}
    }
    sum.round() as i64
}

