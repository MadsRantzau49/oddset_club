mod login;
mod error;
mod render;
mod session;
mod dashboard;
mod settings;
mod insert_money;
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
                settings::change_settings(club_id, club_title, saving_goal, bank_money)
            }
            "/add_user" => {
                if club_id == 0{return render_error("Session died");}
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let color = form_data.get("color").map(String::as_str).unwrap_or("");
                settings::add_user(club_id, username, color)
            }
            "/delete_player" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                settings::delete_user(club_id, user_id)
            }
            "/edit_player" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let username = form_data.get("username").map(String::as_str).unwrap_or("");
                let color = form_data.get("color").map(String::as_str).unwrap_or("");
                settings::edit_user(club_id, username, color, user_id)
            }
            "/logout" => {
                if club_id == 0{return render_error("Session died");}
                if let Some(sid) = &session_id {
                    session::terminate_session(club_id, sid)
                } else {
                    render::get_html("index.html", 0)
                }
            }
            "/insert_money" => {
                if club_id == 0{return render_error("Session died");}
                let user_id = form_data.get("user_id").map(String::as_str).unwrap_or("");
                let amount_str = form_data.get("amount").map(String::as_str).unwrap_or("");
                let amount: f64 = amount_str.parse().unwrap_or(0.0);
                insert_money::insert_money_insertion(user_id, amount, club_id)
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
