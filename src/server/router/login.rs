use tera::Context;
use super::render::{render_template,render_error, render_set_session_cookie};
use super::session::{create_session};
use super::dashboard::{load_dashboard_context};
use crate::database::establish_connection;
use crate::database::club_db::{verify_club,add_club,club_already_exist_db,get_club_id_from_username, add_club_settings};

pub fn login(username: &str, password: &str) -> String {
    let conn: rusqlite::Connection = establish_connection().expect("Failed to connect to DB");
    let mut context = Context::new();

    if verify_club(&conn, username, password).unwrap_or(false) {
        let club_id = get_club_id_from_username(&conn, username);
        match club_id {
            Ok(id) => {
                let session_id = create_session(id);
                context = load_dashboard_context(id);
                return render_set_session_cookie("dashboard.html",&context, session_id);
            },
            Err(_) => {return render_error("Could not start session, please try login again");}
        }
    } else {
        context.insert("error_message", "Wrong username or password!");
        render_template("index.html", &context)
    }
}

pub fn create_club(username: &str, password: &str) -> String{
    let conn = establish_connection().expect("Failed to connect to DB");
    if club_already_exist_db(&conn, username).unwrap_or(false){
        let mut context = Context::new();
        context.insert("error_message", "Club name already exist");
        return render_template("create_club.html", &context);
    }

    if add_club(&conn, username, password).unwrap_or(false){
        match get_club_id_from_username(&conn, username){
            Ok(club_id) => {
                match add_club_settings(&conn,club_id){
                    Ok(_) => return login(username,password),
                    Err(_) => return render_error("Failed to setup club settings")
                }
            },
            Err(_) => {return render_error("Failed to setup club settings");}
        }        
    }
    render_error("Failed to add club")
}

