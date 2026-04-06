use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn generate_token(user_id: &str) -> Result<String, String> {
    let secret =
        std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRETが設定されていません".to_string())?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + 60 * 60 * 24,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("トークン生成エラー: {}", e))
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let secret =
        std::env::var("JWT_SECRET").map_err(|_| "JWT_SECRETが設定されていません".to_string())?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| format!("トークン検証エラー: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn setup() {
        env::set_var("JWT_SECRET", "test_secret_key_for_testing");
    }

    #[test]
    #[serial]
    fn test_generate_and_verify_token() {
        setup();
        let user_id = "test_user";

        let token = generate_token(user_id).expect("トークン生成に失敗しました");
        let claims = verify_token(&token).expect("トークン検証に失敗しました");

        assert_eq!(claims.sub, user_id);
    }

    #[test]
    #[serial]
    fn test_token_expiry_is_24_hours() {
        setup();
        let token = generate_token("user123").unwrap();
        let claims = verify_token(&token).unwrap();

        assert_eq!(claims.exp - claims.iat, 60 * 60 * 24);
    }

    #[test]
    #[serial]
    fn test_verify_invalid_token() {
        setup();
        let result = verify_token("invalid.token.string");

        assert!(result.is_err());
    }

    #[test]
    #[serial]
    fn test_verify_token_with_wrong_secret() {
        // 別のシークレットで生成したトークンは検証できない
        env::set_var("JWT_SECRET", "secret_a");
        let token = generate_token("user_x").unwrap();

        env::set_var("JWT_SECRET", "secret_b");
        let result = verify_token(&token);

        // テスト後にシークレットを元に戻す
        env::set_var("JWT_SECRET", "test_secret_key_for_testing");

        assert!(result.is_err());
    }
}
