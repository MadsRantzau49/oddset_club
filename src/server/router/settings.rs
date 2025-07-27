use tera::Context;
use crate::database::establish_connection;
use crate::database::club_db::{get_club_saving_goals_from_id,change_settings_db};
use crate::database::players_db::{get_players_from_club_id};
use crate::server::router::render::{render_template};

pub fn render_settings(club_id: i64) -> String {
    let context = get_settings_context(club_id);
    return render_template("settings.html", &context);        
}

pub fn change_settings(club_id: i64, club_title: &str, saving_goal: f64, bank_money: f64) -> String{
    let conn = establish_connection().expect("Failed to connect to DB");
    match change_settings_db(&conn, club_id, club_title,saving_goal,bank_money) {
        Ok(_) => {
            let mut context = get_settings_context(club_id);
            context.insert("message", "Success changing the settings");
            return render_template("settings.html", &context);
        },
        Err(_) => {
            let mut context = get_settings_context(club_id);
            context.insert("message", "Failure changing settings");
            return render_template("settings.html", &context);
        }
    }
}

fn get_settings_context(club_id: i64) -> Context{
    let mut context = Context::new();
    let conn = establish_connection().expect("Failed to connect to DB");
    
    if club_id >= 0{
        match get_club_saving_goals_from_id(&conn, club_id) {
            Ok(saving_goal) => {
                context.insert("bank_money", &saving_goal.money_current_bank);
                context.insert("club_title", &saving_goal.title);
                context.insert("saving_goal", &saving_goal.money_goal);
            },
            Err(_) => {}
        }
        match get_players_from_club_id(&conn, club_id) {
            Ok(users) => {
                context.insert("users",&users);
            }
            Err(_) => {}
        }
        return context;
    } else {
        return context;
    }
}