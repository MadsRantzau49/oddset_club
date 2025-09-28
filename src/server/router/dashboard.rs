use tera::Context;
use crate::database::establish_connection;
use crate::database::club_db::{get_club_settings_from_id};
use super::debt::{get_number_of_unpaid_debts};
use super::odds::{get_number_of_unresolved_odds};
use super::statistics::{get_total_money};

pub fn load_dashboard_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to DB");
    let mut context = Context::new();
    match get_club_settings_from_id(&conn, club_id){
        Ok(setting) => {
            let current_money: f64 = get_total_money(&conn,club_id) as f64;
            let progress_percent = if setting.money_goal > 0.0 {
                (current_money / setting.money_goal) * 100.0
            } else {
                0.0  
            };
            println!("{current_money}   {progress_percent}");
            
            context.insert("progress_percent", &progress_percent.round()); 
            context.insert("current_money", &get_total_money(&conn,club_id));
            context.insert("goal_money", &setting.money_goal);
            context.insert("goal_title", &setting.title);
            context.insert("missing_results", &get_number_of_unresolved_odds(club_id));       
            context.insert("unresolved_bets", &get_number_of_unpaid_debts(club_id)); 
            return context;
        },
        Err(_) => {return context;}
    }
}