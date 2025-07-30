use rusqlite::{Connection, Result, params};
use super::database_structs::{Debt};

pub fn get_debt_from_club_id(conn: &Connection, club_id: i64) -> Result<Vec<Debt>> {
    let mut stmt = conn.prepare("
    SELECT 
        debts.id,
        users.username,
        users.color,
        debts.amount,
        debts.description,
        debts.is_paid,
        debts.created_at
    FROM 
        debts
    JOIN 
        users ON debts.user_id = users.id
    WHERE 
        users.club_id = ?1
    ORDER BY 
        debts.created_at DESC;
    ")?;
    
    
    let insertion_iter = stmt.query_map([club_id], |row| {
        Ok(Debt {
            id: row.get(0)?,
            username: row.get(1)?,
            color: row.get(2)?,
            amount: row.get(3)?,
            description: row.get(4)?,
            is_paid: row.get(5)?,
            created_at: row.get(6)?,
        })
    })?;
    let mut insertions = Vec::new();
    for insertion in insertion_iter {
        insertions.push(insertion?);
    }
    Ok(insertions)
}

pub fn insert_debt_db(conn: &Connection, user_id: &str, amount: f64, description: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO debts (user_id, amount, description, is_paid) VALUES (?1, ?2, ?3, ?4)",
        params![user_id, amount, description, false],
    )?;
    Ok(())
}

pub fn delete_debt_db(conn: &Connection, debt_id: &str, club_id: i64) -> Result<()> {
    conn.execute(
        "
        DELETE FROM debts
        WHERE id = ?1
        AND user_id IN (SELECT id FROM users WHERE club_id = ?2)
        ",
        params![debt_id, club_id],
    )?;
    Ok(())
}

pub fn mark_debt_paid_db(conn: &Connection, debt_id: &str, club_id: i64) -> Result<()> {
    conn.execute(
        "
        UPDATE debts
        SET is_paid = 1
        WHERE id = ?1
          AND EXISTS (
            SELECT 1
            FROM users
            WHERE users.id = debts.user_id
              AND users.club_id = ?2
          )
        ",
        params![debt_id, club_id],
    )?;
    Ok(())
}

pub fn get_number_of_unpaid_debts_db(conn: &Connection, club_id: i64) -> Result<i32> {
    let mut stmt = conn.prepare("
        SELECT COUNT(*) FROM debts
        JOIN users ON users.id = debts.user_id
        WHERE debts.is_paid = 0 AND users.club_id = ?1
    ")?;
    let count: i32 = stmt.query_row([club_id],|row| row.get(0))?;
    Ok(count)
}
