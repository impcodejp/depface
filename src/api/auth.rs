// src/api/auth.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

use crate::middleware::auth::AuthUser;
use crate::models::auth::LoginRequest;
use crate::services::api_service::ApiService;

pub async fn login_handler(
    State(service): State<Arc<ApiService>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match service.login_user(payload).await {
        Ok(response) => (StatusCode::OK, Json(serde_json::json!(response))),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": e })),
        ),
    }
}

pub async fn logout_handler(
    State(service): State<Arc<ApiService>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    match service.logout_user(&auth_user.token).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "message": "ログアウトしました" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e })),
        ),
    }
}
