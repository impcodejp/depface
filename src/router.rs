use axum::{Router, routing::post};
use std::sync::Arc;

use crate::services::api_service::ApiService;
use crate::api::user::register_user_handler;

pub fn create_router(state: Arc<ApiService>) -> Router {
    Router::new()
        .route("/api/users", post(register_user_handler))
        .with_state(state)
}