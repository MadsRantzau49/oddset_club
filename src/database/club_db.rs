use rusqlite::{Connection, Result,params};
use super::database_structs::{SavingGoal};


pub fn add_club(conn: &Connection, username: &str, password: &str) -> Result<bool> {
    conn.execute(
        "INSERT INTO clubs (username, password) VALUES (?1, ?2)",
        [username, password],
    )?;
    Ok(true)
}

pub fn add_club_settings(conn: &Connection, club_id: i64) -> Result<bool> {
    conn.execute(
        "INSERT INTO saving_goals (club_id, money_current_bank, money_current_betting_acount, money_goal, title)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![club_id, 0.0_f64, 0.0_f64, 0.0_f64, "Vacation"],
    )?;
    Ok(true)
}

pub fn verify_club(conn: &Connection, username: &str, password: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM clubs WHERE username = ?1 AND password = ?2")?;
    let count: i64 = stmt.query_row([username, password], |row| row.get(0))?;
    Ok(count == 1)
}

pub fn club_already_exist_db(conn: &Connection, username: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM clubs WHERE username = ?1")?;
    let count: i64 = stmt.query_row([username], |row| row.get(0))?;
    Ok(count >= 1)
}

pub fn get_club_id_from_username(conn: &Connection, username: &str) -> Result<i64> {
    let mut stmt = conn.prepare("SELECT id FROM clubs WHERE username = ?1")?;
    let id: i64 = stmt.query_row([username], |row| row.get(0))?;
    Ok(id)
}


pub fn get_club_saving_goals_from_id(conn: &Connection, club_id: i64) -> Result<SavingGoal> {
    let mut stmt = conn.prepare("SELECT money_current_bank, money_current_betting_acount, money_goal, title FROM saving_goals WHERE club_id = ?1")?;
    let saving_goal = stmt.query_row([club_id], |row| {
        Ok(SavingGoal {
            money_current_bank: row.get(0)?,
            money_current_betting_acount: row.get(1)?,
            money_goal: row.get(2)?,
            title: row.get(3)?,
        })
    })?;
    Ok(saving_goal)
}

pub fn change_settings_db(
    conn: &Connection,
    club_id: i64,
    club_title: &str,
    saving_goal: f64,
    bank_money: f64
) -> Result<bool> {
    conn.execute(
        "UPDATE saving_goals
         SET money_goal = ?1, money_current_bank = ?2, title = ?3
         WHERE club_id = ?4",
        params![saving_goal, bank_money, club_title, club_id],
    )?;
    Ok(true)
}