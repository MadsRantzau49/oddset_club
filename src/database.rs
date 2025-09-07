use rusqlite::{Connection, Result};

pub mod club_db;
pub mod database_structs;
pub mod players_db;
pub mod session_db;
pub mod money_insertion_db;
pub mod debt_db;
pub mod odds_db;
pub mod sql;

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
            potential_win REAL NOT NULL,
            description TEXT,
            result INTEGER NOT NULL,
            is_volunteer_bet BOOLEAN,
            is_gain_freebet BOOLEAN,
            is_freebet BOOLEAN,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        [],
    )?;
    // money insertions
    conn.execute(
        "CREATE TABLE IF NOT EXISTS money_insertions (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            amount REAL,
            is_valid_balance BOOLEAN,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // saving goals
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            club_id INTEGER,
            money_current_bank REAL,
            money_current_betting_acount REAL,
            money_goal REAL,
            title TEXT,
            default_stake REAL,
            statistics_start_date TEXT DEFAULT '2000-01-01',
            statistics_end_date TEXT DEFAULT (DATE('now')),
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
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
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

