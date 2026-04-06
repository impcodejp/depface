use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    http::{request::Parts, StatusCode},
    Json,
};
use serde_json::json;
use std::sync::Arc;

use crate::auth::verify_token;
use crate::services::api_service::ApiService;

pub struct AuthUser {
    pub user_id: String,
    pub token: String,
}

fn auth_error(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::UNAUTHORIZED, Json(json!({ "error": msg })))
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<ApiService>: FromRef<S>,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // トークンの抽出
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| auth_error("認証トークンがありません"))?
            .to_string();

        // トークンの検証
        let claims =
            verify_token(&token).map_err(|_| auth_error("無効または期限切れのトークンです"))?;

        // ブラックリストチェック
        let State(service): State<Arc<ApiService>> = State::from_request_parts(parts, state)
            .await
            .map_err(|_| auth_error("サーバーエラー"))?;

        let is_blacklisted = service
            .blacklist_repo
            .is_blacklisted(&token)
            .await
            .map_err(|_| auth_error("サーバーエラー"))?;

        if is_blacklisted {
            return Err(auth_error("このトークンは無効化されています"));
        }

        Ok(AuthUser {
            user_id: claims.sub,
            token,
        })
    }
}
