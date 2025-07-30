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
#[derive(Debug, Serialize)]
pub struct MoneyInsertion {
    pub id: i64,
    pub username: String,
    pub color: String,
    pub amount: f64,
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