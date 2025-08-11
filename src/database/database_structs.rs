use serde::Serialize;

pub struct SavingGoal {
    pub money_current_bank: f64,
    pub money_current_betting_acount: f64,
    pub money_goal: f64,
    pub title: String,
    pub default_stake: f64,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub color: String,
}
#[derive(Debug, Serialize)]
pub struct MoneyInsertion {
    pub id: i64,
    pub username: String,
    pub color: String,
    pub amount: f64,
    pub is_valid_balance: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct Debt{
    pub id: i64, 
    pub username: String,
    pub color: String,
    pub amount: f64,
    pub description: String,
    pub is_paid: bool,
    pub created_at: String,
}


#[derive(Debug, Serialize)]
pub struct Odds{
    pub id: i64,
    pub user_id: i64,
    pub username: String,
    pub color: String,
    pub stake: f64,
    pub odds: f64,
    pub potential_win: f64,
    pub description: String, 
    pub result: i64,
    pub is_volunteer_bet: bool,
    pub is_gain_freebet: bool,
    pub is_freebet: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserStatistic{
    pub username: String,
    pub color: String,
    pub total_balance: i64,
    pub total_won: i64,
    pub winrate: f64,
    pub total_deposit: i64,
    pub amount_of_freebets: i64,
}

#[derive(Debug, Serialize)]
pub struct OddsStatistic{
        pub stake: f64,
        pub odds: f64,
        pub potential_win: f64,
        pub result: i64,
        pub is_volunteer_bet: bool,
        pub is_gain_freebet: bool,
}
