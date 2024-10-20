use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;

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
    pub merchant_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRegister {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub password: String,
    pub user_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserData {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub user_type: String,
    pub status: String,
    pub merchant_id: Option<i32>,
}