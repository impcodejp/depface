// src/services/api_service/mod.rs

use crate::db::repository::user::UserRepository;
use crate::db::token_blacklist::TokenBlacklistRepository;

pub mod auth;
pub mod user;

pub struct ApiService {
    pub user_repo: UserRepository,
    pub blacklist_repo: TokenBlacklistRepository,
}

impl ApiService {
    pub fn new(user_repo: UserRepository, blacklist_repo: TokenBlacklistRepository) -> Self {
        Self {
            user_repo,
            blacklist_repo,
        }
    }
}
