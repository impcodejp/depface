// src/db/valkey.rs

use redis::{AsyncCommands, Client};
use std::env;

pub type ValkeyClient = Client;

pub fn establish_valkey_connection() -> ValkeyClient {
    let valkey_url = env::var("VALKEY_URL").expect("VALKEY_URL must be set in .env");
    Client::open(valkey_url).expect("Invalid Valkey URL")
}

pub struct ValkeyRepository {
    client: Client,
}

impl ValkeyRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    // トークンをブラックリストに追加（ttl_seconds 秒後に自動削除）
    pub async fn blacklist_token(&self, token: &str, ttl_seconds: i64) -> Result<(), String> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Valkey接続エラー: {}", e))?;

        let key = format!("blacklist:{}", token);
        conn.set_ex::<_, _, ()>(&key, "1", ttl_seconds as u64)
            .await
            .map_err(|e| format!("Valkey書き込みエラー: {}", e))
    }

    // トークンがブラックリストに存在するか確認
    pub async fn is_blacklisted(&self, token: &str) -> Result<bool, String> {
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| format!("Valkey接続エラー: {}", e))?;

        let key = format!("blacklist:{}", token);
        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| format!("Valkey読み取りエラー: {}", e))?;

        Ok(exists)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    fn setup_repo() -> ValkeyRepository {
        dotenv().ok();
        let client = establish_valkey_connection();
        ValkeyRepository::new(client)
    }

    #[tokio::test]
    async fn test_blacklist_and_check_token() {
        let repo = setup_repo();
        let token = "test_blacklist_token_abc123";

        // 追加前はブラックリストに存在しない
        let before = repo.is_blacklisted(token).await.unwrap();
        assert!(!before, "追加前にブラックリストに存在しています");

        // ブラックリストに追加（10秒後に自動削除）
        repo.blacklist_token(token, 10).await.unwrap();

        // 追加後は存在する
        let after = repo.is_blacklisted(token).await.unwrap();
        assert!(after, "追加後にブラックリストに存在していません");
    }

    #[tokio::test]
    async fn test_non_existing_token_is_not_blacklisted() {
        let repo = setup_repo();
        let result = repo.is_blacklisted("non_existing_token_xyz_999").await.unwrap();

        assert!(!result);
    }
}
