use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
    async_trait, // 追加
};
use serde_json::json;

use crate::auth::verify_token;

pub struct AuthUser {
    pub user_id: String,
}

// エラーレスポンスを生成するヘルパー
fn auth_error(msg: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": msg })),
    )
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // トークンの抽出
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| auth_error("認証トークンがありません"))?;

        // トークンの検証
        let claims = verify_token(token).map_err(|_| {
            auth_error("無効または期限切れのトークンです")
        })?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}