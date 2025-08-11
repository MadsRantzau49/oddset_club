use rusqlite::{Connection, Result, params};

use crate::database::database_structs::{Odds, OddsStatistic};

pub fn insert_odds_db(conn: &Connection, user_id: &str, stake: f64, odds: f64, potential_win: f64, description: &str, result: i64, is_volunteer_bet: bool, is_gain_freebet: bool, is_freebet: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO odds (user_id, stake, odds, potential_win, description, result, is_volunteer_bet, is_gain_freebet, is_freebet ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![user_id, stake, odds, potential_win, description, result as i64, is_volunteer_bet, is_gain_freebet, is_freebet],
    )?;
    Ok(())
}

pub fn get_all_odds_data_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<Odds>> {
    let mut stmt = conn.prepare("
    SELECT 
        odds.id,
        users.username,
        users.color,
        odds.stake,
        odds.odds,
        odds.potential_win,
        odds.description,
        odds.result,
        odds.is_volunteer_bet,
        odds.is_gain_freebet,
        odds.created_at,
        users.id,
        odds.is_freebet
    FROM 
        odds
    JOIN 
        users ON odds.user_id = users.id
    WHERE 
        users.club_id = ?1
    ORDER BY 
        odds.created_at DESC;
    ")?;
    
    
    let odds_iter = stmt.query_map([club_id], |row| {
        Ok(Odds {
            id: row.get(0)?,
            user_id: row.get(11)?,
            username: row.get(1)?,
            color: row.get(2)?,
            stake: row.get(3)?,
            odds: row.get(4)?,
            potential_win: row.get(5)?,
            description: row.get(6)?,
            result: row.get(7)?,
            is_volunteer_bet: row.get(8)?,
            is_gain_freebet: row.get(9)?,
            is_freebet: row.get(12)?,
            created_at: row.get(10)?,
        })

    })?;
    let mut oddss = Vec::new();
    for odds in odds_iter {
        oddss.push(odds?);
    }
    Ok(oddss)
}

pub fn get_user_odds_data(conn: &Connection, user_id: i64) -> Result<Vec<OddsStatistic>> {
    let mut stmt = conn.prepare("
    SELECT 
        stake,
        odds,
        potential_win,
        result,
        is_volunteer_bet,
        is_gain_freebet
    FROM 
        odds
    WHERE 
        user_id = ?1;
    ")?;
    
    
    let odds_iter = stmt.query_map([user_id], |row| {
        Ok(OddsStatistic {
            stake: row.get(0)?,
            odds: row.get(1)?,
            potential_win: row.get(2)?,
            result: row.get(3)?,
            is_volunteer_bet: row.get(4)?,
            is_gain_freebet: row.get(5)?,
        })

    })?;
    let mut oddss = Vec::new();
    for odds in odds_iter {
        oddss.push(odds?);
    }
    Ok(oddss)
}


pub fn insert_result_db(conn: &Connection, club_id: i64, odds_id: &str, result: i64) -> Result<()>{
    conn.execute(
        "
        UPDATE odds
        SET result = ?1
        FROM users
        WHERE odds.user_id = users.id
        AND users.club_id = ?2
        AND odds.id = ?3;
  ",
        params![result, club_id, odds_id],
    )?;
    Ok(())
}

pub fn get_number_of_unresolved_odds_db(conn: &Connection, club_id: i64) -> Result<i32> {
    let mut stmt = conn.prepare("
        SELECT COUNT(*) FROM odds
        JOIN users ON users.id = odds.user_id
        WHERE odds.result = 0 AND users.club_id = ?1
    ")?;
    let count: i32 = stmt.query_row([club_id],|row| row.get(0))?;
    Ok(count)
}


pub fn delete_odds_db(conn: &Connection, club_id: i64, odds_id: &str) -> Result<()> {
    conn.execute(
        "
        DELETE FROM odds
        WHERE id = ?1
        AND user_id IN (
            SELECT id FROM users WHERE club_id = ?2
        );
        ", 
        params![odds_id, club_id],
    )?;
    Ok(())
}

pub fn get_oldest_odds(conn: &Connection, club_id: i64) -> Result<String> {
    let mut stmt = conn.prepare("
        SELECT created_at
        FROM odds
        WHERE user_id IN (SELECT id FROM users WHERE club_id = ?1)
        ORDER BY created_at ASC
        LIMIT 1
    ")?;

    let result: Result<String> = stmt.query_row([club_id], |row| row.get(0));
    result
}

pub fn insert_sheet_odds_db(conn: &Connection, user_id: i64, stake: f64, odds: f64, potential_win: f64, description: String, result: i64, is_volunteer_bet: bool, is_gain_freebet: bool, created_at: String, is_freebet: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO odds (user_id, stake, odds, potential_win, description, result, is_volunteer_bet, is_gain_freebet, created_at, is_freebet ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![user_id, stake, odds, potential_win, description, result, is_volunteer_bet, is_gain_freebet, created_at, is_freebet],
    )?;
    Ok(())
}

pub fn delete_all_odds(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM odds",
        [],
    )?;
    Ok(())
}