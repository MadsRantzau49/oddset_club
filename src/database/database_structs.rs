use serde::Serialize;

pub struct SavingGoal {
    pub money_current_bank: f64,
    pub money_current_betting_acount: f64,
    pub money_goal: f64,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub color: String,
}