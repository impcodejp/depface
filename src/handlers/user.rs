use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;
use tracing::{info, warn};

use crate::middleware::auth::AuthUser;
use crate::models::user::CreateUserRequestFromFrontend;
use crate::services::AppService;

pub async fn register_user_handler(
    _auth_user: AuthUser,
    State(service): State<Arc<AppService>>,
    Json(payload): Json<CreateUserRequestFromFrontend>,
) -> impl IntoResponse {
    info!(user_id = %payload.user_id, "ユーザー登録リクエスト受信");
    match service.register_user(payload).await {
        Ok(user_response) => {
            info!(user_id = %user_response.user_id, "ユーザー登録成功");
            (StatusCode::CREATED, Json(user_response)).into_response()
        }
        Err(err_msg) => {
            warn!(error = %err_msg, "ユーザー登録失敗");
            (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": err_msg })),
            )
                .into_response()
        }
    }
}
