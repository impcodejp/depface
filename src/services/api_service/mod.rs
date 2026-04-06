// src/services/api_service/mod.rs

use crate::db::repository::user::UserRepository;

pub mod auth;
pub mod user;

pub struct ApiService {
    user_repo: UserRepository,
    // 今後ここに valkey_pool: redis::Client などを追加します
}

impl ApiService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }
}