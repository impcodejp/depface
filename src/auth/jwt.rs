// src/auth/jwt.rs

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// JWTのペイロード（クレーム）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // subject（user_id を入れる）
    pub exp: usize,    // expiration（有効期限、Unixタイムスタンプ）
    pub iat: usize,    // issued at（発行時刻）
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRETが設定されていません".to_string())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + 60 * 60 * 24, // 24時間後
    };

    encode(
        &Header::default(), // デフォルトはHS256
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("トークン生成エラー: {}", e))
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| "JWT_SECRETが設定されていません".to_string())?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("トークン検証エラー: {}", e))
}
