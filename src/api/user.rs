// src/api/user.rs

use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use crate::services::api_service::ApiService;
use crate::models::user::CreateUserRequestFromFrontend;

/// ユーザー登録ハンドラー
pub async fn register_user_handler(
    State(service): State<Arc<ApiService>>,
    Json(payload): Json<CreateUserRequestFromFrontend>,
) -> impl IntoResponse {
    match service.register_user(payload).await {
        Ok(user_response) => (
            StatusCode::CREATED,
            Json(user_response),
        ).into_response(),
        Err(err_msg) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": err_msg })),
        ).into_response(),
    }
}