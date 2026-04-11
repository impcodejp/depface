// src/api/auth.rs

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{info, warn};

use crate::middleware::auth::AuthUser;
use crate::models::auth::LoginRequest;
use crate::services::api_service::ApiService;

pub async fn login_handler(
    State(service): State<Arc<ApiService>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    info!(user_id = %payload.user_id, "ログインリクエスト受信");
    match service.login_user(payload).await {
        Ok(response) => {
            info!(user_id = %response.user_id, "ログイン成功");
            (StatusCode::OK, Json(serde_json::json!(response)))
        }
        Err(e) => {
            warn!(error = %e, "ログイン失敗");
            (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": e })))
        }
    }
}

pub async fn logout_handler(
    State(service): State<Arc<ApiService>>,
    auth_user: AuthUser,
) -> impl IntoResponse {
    info!(user_id = %auth_user.user_id, "ログアウトリクエスト受信");
    match service.logout_user(&auth_user.token).await {
        Ok(_) => {
            info!(user_id = %auth_user.user_id, "ログアウト成功");
            (StatusCode::OK, Json(serde_json::json!({ "message": "ログアウトしました" })))
        }
        Err(e) => {
            warn!(user_id = %auth_user.user_id, error = %e, "ログアウト失敗");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e })))
        }
    }
}
