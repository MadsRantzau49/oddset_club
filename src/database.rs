use rusqlite::{Connection, Result};

pub mod club_db;
pub mod database_structs;
pub mod players_db;
pub mod session_db;

pub fn establish_connection() -> Result<Connection> {
    Connection::open("data/database.db")
}

pub fn init_db(conn: &Connection) -> Result<()> {
    // clubs
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clubs (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        [],
    )?;

    // users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            color TEXT DEFAULT '#0f63a8ff',
            club_id INTEGER,
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )",
        [],
    )?;

    // odds
    conn.execute(
        "CREATE TABLE IF NOT EXISTS odds (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            stake REAL NOT NULL,
            odds REAL NOT NULL,
            potential_win REAL,
            description TEXT,
            result INTEGER NOT NULL,
            volunteer_bet BOOLEAN,
            gain_freebet BOOLEAN,
            created_at TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        [],
    )?;
    // money insertions
    conn.execute(
        "CREATE TABLE IF NOT EXISTS money_insertions (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            amount REAL,
            created_at TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        [],
    )?;

    // saving goals
    conn.execute(
        "CREATE TABLE IF NOT EXISTS saving_goals (
            id INTEGER PRIMARY KEY,
            club_id INTEGER,
            money_current_bank REAL,
            money_current_betting_acount REAL,
            money_goal REAL,
            title TEXT,
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )",
        [],
    )?;

    // debts
    conn.execute(
        "CREATE TABLE IF NOT EXISTS debts (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            amount REAL,
            description TEXT,
            is_paid BOOLEAN,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )",
        [],
    )?;

    // Sessions
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            session_id TEXT NOT NULL,
            club_id INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [])?;
    Ok(())
}

