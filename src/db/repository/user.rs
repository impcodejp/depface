// src/db/repository/user.rs

use sqlx::PgPool;
use crate::models::user::{ User, CreateUserRequest };

pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ユーザーの新規追加
    pub async fn create_user(&self, user: CreateUserRequest) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO main.users (user_id, user_name, email, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, user_name, email, password_hash, created_at, updated_at
            "#,
            user.user_id,
            user.user_name,
            user.email,
            user.password_hash
        )
        .fetch_one(&self.pool)
        .await
    }

    // user_id でユーザーを検索
    pub async fn find_by_user_id(&self, user_id: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, user_id, user_name, email, password_hash, created_at, updated_at
        FROM main.users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(&self.pool)
    .await
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::pool::establish_connection;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_user_repository_flow() {
        dotenv().ok();
        let pool = establish_connection().await;
        let repo = UserRepository::new(pool);

        let unique_id: &str = "test_user_1";
        // テスト前処理として、test_user_1 という user_id を削除する
        println!("テスト前処理: user_id '{}' を削除します", unique_id);
        let _ = sqlx::query!(
            r#"
            DELETE FROM main.users WHERE user_id = $1
            "#,
            unique_id
        )
        .execute(&repo.pool)
        .await;

        // 1. ユーザー新規追加テスト

        // 1.1. テストデータの準備
        let test_user1 = CreateUserRequest {
            user_id: format!("{}", unique_id),
            user_name: "テストユーザー".to_string(),
            email: format!("{}@example.com", unique_id),
            password_hash: "secret_hash".to_string(),
        };

        // 1.2. 実行
        let result = repo.create_user(test_user1).await;

        // 1.3. 検証
        assert!(result.is_ok(), "ユーザー作成に失敗しました: {:?}", result.err());
        
        let created_user = result.unwrap();
        assert_eq!(created_user.user_name, "テストユーザー");
        assert!(created_user.id > 0); // IDが自動採番されているか

        // 2. 重複ユーザーIDのテスト
        // 2.1. テストデータの準備
        let test_user2 = CreateUserRequest {
            user_id: format!("{}", unique_id), // 同じuser_idを使用
            user_name: "テストユーザー2".to_string(),
            email: format!("{}_2@example.com", unique_id),
            password_hash: "secret_hash".to_string(),
        };

        // 2.2. 実行
        let result2 = repo.create_user(test_user2).await;

        // 2.3. 検証
        assert!(result2.is_err(), "重複したuser_idでユーザー作成が成功してしまいました");
        
        // 3. 重複メールアドレスのテスト
        let test_user3 = CreateUserRequest {
            user_id: format!("{}3", unique_id), // 異なるuser_idを使用
            user_name: "テストユーザー3".to_string(),
            email: format!("{}@example.com", unique_id), // 同じemailを使用
            password_hash: "secret_hash".to_string(),
        };

        // 3.2. 実行
        let result3 = repo.create_user(test_user3).await;

        // 3.3. 検証
        assert!(result3.is_err(), "重複したemailでユーザー作成が成功してしまいました");

    }
}