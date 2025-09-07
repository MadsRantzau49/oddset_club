use rusqlite::{Connection, Result, params};

pub fn run_sql(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
    "
            UPDATE odds
            SET is_gain_freebet = 0
            WHERE is_gain_freebet = 1 AND id != 5;
        ",
        params![],
    )?;
    Ok(())
}
