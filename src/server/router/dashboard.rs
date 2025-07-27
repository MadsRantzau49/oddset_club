use tera::Context;
use crate::database::establish_connection;
use crate::database::club_db::{get_club_saving_goals_from_id};


pub fn load_dashboard_context(club_id: i64) -> Context {
    let conn = establish_connection().expect("Failed to connect to DB");
    let mut context = Context::new();
    match get_club_saving_goals_from_id(&conn, club_id){
        Ok(saving_goal) => {
            let current_money = saving_goal.money_current_bank + saving_goal.money_current_betting_acount;
            let progress_percent = if saving_goal.money_goal > 0.0 {
                (current_money / saving_goal.money_goal) * 100.0
            } else {
                0.0  
            };
            
            context.insert("progress_percent", &progress_percent); 
            context.insert("current_money", &current_money);
            context.insert("goal_money", &saving_goal.money_goal);
            context.insert("goal_title", &saving_goal.title);
            context.insert("missing_results", &2);       
            context.insert("unresolved_bets", &100); 
            return context;
        },
        Err(_) => {return context;}
    }
}