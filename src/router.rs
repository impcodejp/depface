use axum::{Router, routing::{post, get}};
use std::sync::Arc;

use crate::services::api_service::ApiService;
use crate::api::user::register_user_handler;
use crate::api::auth::login_handler;

pub fn create_router(state: Arc<ApiService>) -> Router {
    Router::new()
        // 認証不要のルート
        .route("/api/users", post(register_user_handler))
        .route("/api/auth/login", post(login_handler))
        // 認証必要のルートはここに追加していく
        // .route("/api/protected", get(protected_handler))
        .with_state(state)
}
