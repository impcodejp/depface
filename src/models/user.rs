// src/models/user.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestFromFrontend {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateUserResponseForFrontend {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
}
