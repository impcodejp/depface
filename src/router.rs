use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

use crate::api::auth::{login_handler, logout_handler};
use crate::api::user::register_user_handler;
use crate::services::api_service::ApiService;

pub fn create_router(state: Arc<ApiService>) -> Router {
    Router::new()
        // 認証不要のルート
        .route("/api/users", post(register_user_handler))
        .route("/api/auth/login", post(login_handler))
        // 認証必要のルート
        .route("/api/auth/logout", post(logout_handler))
        .with_state(state)
}
