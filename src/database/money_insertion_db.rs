use rusqlite::{Connection, Result, params};
use super::database_structs::{MoneyInsertion};

pub fn get_money_insertion_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<MoneyInsertion>> {
    let mut stmt = conn.prepare("
    SELECT 
        money_insertions.id,
        users.username,
        users.color,
        money_insertions.amount,
        money_insertions.created_at
    FROM 
        money_insertions
    JOIN 
        users ON money_insertions.user_id = users.id
    WHERE 
        users.club_id = ?1
    ")?;
    
    
    let insertion_iter = stmt.query_map([club_id], |row| {
        Ok(MoneyInsertion {
            id: row.get(0)?,
            username: row.get(1)?,
            color: row.get(2)?,
            amount: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    let mut insertions = Vec::new();
    for insertion in insertion_iter {
        insertions.push(insertion?);
    }
    Ok(insertions)
}

pub fn insert_money_insertion_db(conn: &Connection, user_id: &str, amount: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO money_insertions (user_id, amount) VALUES (?1, ?2)",
        params![user_id, amount],
    )?;
    Ok(())
}
