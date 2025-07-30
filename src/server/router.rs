mod login;
mod error;
mod render;
mod session;
mod dashboard;
mod settings;
mod insert_money;
mod debt;
mod odds;
mod statistics;
use std::collections::HashMap;
use urlencoding::decode;

use crate::server::router::render::render_error;

pub fn route_request(request: &str) -> String {
    let mut parts = request.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    let form_data = parse_urlencoded_form_data(request);
    let session_id = extract_cookie_session(&request);
    let mut club_id: i64 = 0;
    if let Some(sid) = &session_id {
        if let Some(club_id_tmp) = session::get_club_id_from_session(sid) {
            club_id = club_id_tmp;
        }
    }

    if method == "GET" {
        return match path{
            "/login" => render::get_html("index.html", club_id),
            "/" => render::get_html("index.html",club_id),
            "/create_club" => render::get_html("create_club.html",club_id),
            "/settings" => settings::render_settings(club_id),
            "/insert_money" => insert_money::render_insert_money(club_id),
            "/debt" => debt::render_debt(club_id),
            "/add_odds" => odds::render_add_odds(club_id),
            "/insert_result" => odds::render_insert_odds(club_id),
            "/statistics" => statistics::renderer_statistics(club_id),
            _ => render::render_error("Could not find the page your were searching for ://"),
        }
    }else if method == "POST" {
        return match path {
            "/login" => {
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let password = form_data.get("password").map(String::as_str).unwrap_or("");
                login::login(username, password)
            },
            "/create_club" => {
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let password = form_data.get("password").map(String::as_str).unwrap_or("");
                login::create_club(username, password)
            },
            "/update_club_settings" => {
                if club_id == 0{return render_error("Session died");}
                let club_title = form_data.get("club_title").map(String::as_str).unwrap_or("");
                let saving_goal_str = form_data.get("saving_goal").map(String::as_str).unwrap_or("");
                let saving_goal: f64 = saving_goal_str.parse().unwrap_or(0.0);

                let bank_money_str = form_data.get("bank_money").map(String::as_str).unwrap_or("");
                let bank_money: f64 = bank_money_str.parse().unwrap_or(0.0);

                let default_stake = get_f64(&form_data, "default_stake");
                settings::change_settings(club_id, club_title, saving_goal, bank_money, default_stake)
            },
            "/add_user" => {
                if club_id == 0{return render_error("Session died");}
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let color = form_data.get("color").map(String::as_str).unwrap_or("");
                settings::add_user(club_id, username, color)
            },
            "/delete_player" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                settings::delete_user(club_id, user_id)
            },
            "/edit_player" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let color = form_data.get("color").map(String::as_str).unwrap_or("");
                settings::edit_user(club_id, username, color, user_id)
            },
            "/logout" => {
                if club_id == 0{return render_error("Session died");}
                if let Some(sid) = &session_id {
                    session::terminate_session(club_id, sid)
                } else {
                    render::get_html("index.html", 0)
                }
            },
            "/insert_money" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let amount_str = form_data.get("amount").map(String::as_str).unwrap_or("");
                let amount: f64 = amount_str.parse().unwrap_or(0.0);
                insert_money::insert_money_insertion(user_id, amount, club_id)
            },
            "/delete_insertion" => {
                if club_id == 0{return render_error("Session died");}
                let insertion_id = form_data.get("insertion_id").map(String::as_str).unwrap_or("");
                insert_money::delete_insertion(insertion_id, club_id)
            },
            "/add_debt" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let amount_str = form_data.get("amount").map(String::as_str).unwrap_or("");
                let amount: f64 = amount_str.parse().unwrap_or(0.0);
                let description = form_data.get("description").map(String::as_str).unwrap_or("");
                debt::insert_debt(user_id, amount, description, club_id)
            },
            "/mark_paid" => {
                if club_id == 0{return render_error("Session died");}
                let debt_id = form_data.get("debt_id").map(String::as_str).unwrap_or("");
                debt::mark_debt_paid(debt_id, club_id)
            },
            "/delete_debt" => {
                if club_id == 0{return render_error("Session died");}
                let debt_id = form_data.get("debt_id").map(String::as_str).unwrap_or("");
                debt::delete_debt(debt_id, club_id)
            },
            "/add_odds" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let description = form_data.get("description").map(String::as_str).unwrap_or("");
                let stake = get_f64(&form_data, "stake");
                let odds = get_f64(&form_data, "odds");
                let potential_win = get_f64(&form_data, "potential_win");
                let is_volunteer_bet = get_bool(&form_data, "volunteer_bet");
                let is_gain_freebet = get_bool(&form_data, "gain_freebet");

                odds::insert_odds(club_id, user_id, stake, odds, potential_win, description, is_volunteer_bet, is_gain_freebet)
            },
            "/update_result" => {
                if club_id == 0{return render_error("Session died");}
                let odds_id = form_data.get("odds_id").map(String::as_str).unwrap_or("");
                let result = get_i64(&form_data, "result");    
                odds::insert_result(club_id, odds_id, result)
            },
            "/delete_odds" => {
                if club_id == 0{return render_error("Session died");}
                let odds_id = form_data.get("odds_id").map(String::as_str).unwrap_or("");
                odds::delete_odds(club_id, odds_id)
            }
            _ => error::not_found(),
        
        }


    } else {
        "HTTP/1.1 404 NOT FOUND\r\n\r\n404 Not Found".to_string()
    }
}

fn parse_urlencoded_form_data(request: &str) -> HashMap<String, String> {
    let mut form_data = HashMap::new();

    // Find the body (after headers)
    let body_start = match request.find("\r\n\r\n") {
        Some(pos) => pos + 4,
        None => return form_data,
    };
    let body = &request[body_start..];

    // Split key=value pairs
    for pair in body.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let key = decode(key).unwrap_or_default();
            let value = decode(value).unwrap_or_default();
            form_data.insert(key.to_string(), value.to_string());
        }
    }

    form_data
}

fn extract_cookie_session(request: &str) -> Option<String> {
    for line in request.lines() {
        if line.starts_with("Cookie:") {
            let cookies = line.trim_start_matches("Cookie:").split(';');
            for cookie in cookies {
                let cookie = cookie.trim();
                if let Some((key, val)) = cookie.split_once('=') {
                    if key == "session_id" {
                        return Some(val.to_string());
                    }
                }
            }
        }
    }
    None
}


fn get_f64(form_data: &HashMap<String, String>, key: &str) -> f64 {
    form_data.get(key).and_then(|s| s.parse().ok()).unwrap_or(0.0)
}

fn get_i64(form_data: &HashMap<String, String>, key: &str) -> i64 {
    form_data.get(key).and_then(|s| s.parse().ok()).unwrap_or(0)
}

fn get_bool(form_data: &HashMap<String, String>, key: &str) -> bool {
    form_data.get(key).map(|v| v == "true" || v == "on" || v == "1").unwrap_or(false)
}
