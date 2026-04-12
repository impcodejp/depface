use crate::db::token_blacklist::TokenBlacklistRepository;
use crate::db::user::UserRepository;

pub struct AppService {
    pub user_repo: UserRepository,
    pub blacklist_repo: TokenBlacklistRepository,
}

impl AppService {
    pub fn new(user_repo: UserRepository, blacklist_repo: TokenBlacklistRepository) -> Self {
        Self {
            user_repo,
            blacklist_repo,
        }
    }
}
