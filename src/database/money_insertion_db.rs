use rusqlite::{Connection, Result, params};
use super::database_structs::{MoneyInsertion};

pub fn get_money_insertion_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<MoneyInsertion>> {
    let mut stmt = conn.prepare("
    SELECT 
        money_insertions.id,
        users.username,
        users.color,
        money_insertions.amount,
        money_insertions.is_valid_balance,
        money_insertions.created_at
    FROM 
        money_insertions
    JOIN 
        users ON money_insertions.user_id = users.id
    WHERE 
        users.club_id = ?1
    ORDER BY 
        money_insertions.created_at DESC;
    ")?;
    
    
    let insertion_iter = stmt.query_map([club_id], |row| {
        Ok(MoneyInsertion {
            id: row.get(0)?,
            username: row.get(1)?,
            color: row.get(2)?,
            amount: row.get(3)?,
            is_valid_balance: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    let mut insertions = Vec::new();
    for insertion in insertion_iter {
        insertions.push(insertion?);
    }
    Ok(insertions)
}

pub fn get_user_money_insertions(conn: &Connection, user_id: i64) -> Result<Vec<f64>> {
    let mut stmt = conn.prepare("
        SELECT amount
        FROM money_insertions
        WHERE user_id = ?1;
    ")?;

    let amount_iter = stmt.query_map([user_id], |row| {
        row.get(0)  // get amount directly as f64 (or f32 if you prefer)
    })?;

    let mut amounts = Vec::new();
    for amount in amount_iter {
        amounts.push(amount?);
    }

    Ok(amounts)
}


pub fn insert_money_insertion_db(conn: &Connection, user_id: &str, amount: f64, is_valid_balance: bool) -> Result<()> {
    conn.execute(
        "INSERT INTO money_insertions (user_id, amount, is_valid_balance) VALUES (?1, ?2, ?3)",
        params![user_id, amount, is_valid_balance],
    )?;
    Ok(())
}

pub fn delete_money_insertion_db(conn: &Connection, insertion_id: &str, club_id: i64) -> Result<()> {
    conn.execute(
        "
        DELETE FROM money_insertions
     WHERE id = ?1
       AND user_id IN (SELECT id FROM users WHERE club_id = ?2)
        ",
        params![insertion_id, club_id],
    )?;
    Ok(())
}