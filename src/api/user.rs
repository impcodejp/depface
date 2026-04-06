// src/api/user.rs

use crate::models::user::CreateUserRequestFromFrontend;
use crate::services::api_service::ApiService;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

/// ユーザー登録ハンドラー
pub async fn register_user_handler(
    State(service): State<Arc<ApiService>>,
    Json(payload): Json<CreateUserRequestFromFrontend>,
) -> impl IntoResponse {
    match service.register_user(payload).await {
        Ok(user_response) => (StatusCode::CREATED, Json(user_response)).into_response(),
        Err(err_msg) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": err_msg })),
        )
            .into_response(),
    }
}
