use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub user_type: String,
    pub status: String,
}