// src/db/token_blacklist.rs

use chrono::{Duration, Utc};
use sqlx::PgPool;

pub struct TokenBlacklistRepository {
    pool: PgPool,
}

impl TokenBlacklistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // トークンをブラックリストに追加
    pub async fn blacklist_token(&self, token: &str, ttl_seconds: i64) -> Result<(), String> {
        let expires_at = Utc::now() + Duration::seconds(ttl_seconds);

        sqlx::query!(
            r#"
            INSERT INTO main.token_blacklist (token, expires_at)
            VALUES ($1, $2)
            ON CONFLICT (token) DO NOTHING
            "#,
            token,
            expires_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| format!("ブラックリスト追加エラー: {}", e))?;

        Ok(())
    }

    // トークンがブラックリストに存在するか確認
    pub async fn is_blacklisted(&self, token: &str) -> Result<bool, String> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM main.token_blacklist
                WHERE token = $1 AND expires_at > NOW()
            ) AS "exists!"
            "#,
            token
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("ブラックリスト確認エラー: {}", e))?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::establish_connection;
    use dotenvy::dotenv;

    async fn setup_repo() -> TokenBlacklistRepository {
        dotenv().ok();
        let pool = establish_connection().await;
        TokenBlacklistRepository::new(pool)
    }

    #[tokio::test]
    async fn test_blacklist_and_check_token() {
        let repo = setup_repo().await;
        let token = "test_blacklist_token_abc123";

        // テスト前にクリーンアップ
        sqlx::query!("DELETE FROM main.token_blacklist WHERE token = $1", token)
            .execute(&repo.pool)
            .await
            .unwrap();

        // 追加前はブラックリストに存在しない
        let before = repo.is_blacklisted(token).await.unwrap();
        assert!(!before, "追加前にブラックリストに存在しています");

        // ブラックリストに追加
        repo.blacklist_token(token, 60).await.unwrap();

        // 追加後は存在する
        let after = repo.is_blacklisted(token).await.unwrap();
        assert!(after, "追加後にブラックリストに存在していません");
    }

    #[tokio::test]
    async fn test_non_existing_token_is_not_blacklisted() {
        let repo = setup_repo().await;
        let result = repo
            .is_blacklisted("non_existing_token_xyz_999")
            .await
            .unwrap();

        assert!(!result);
    }
}
