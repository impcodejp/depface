use crate::models::user::{CreateUserRequest, User};
use sqlx::PgPool;

pub struct UserRepository {
    pub pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

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

    pub async fn count_users(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT COUNT(*) as count FROM main.users
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.count.unwrap_or(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn test_user_repository_flow() {
        dotenv().ok();
        let pool = establish_connection().await;
        let repo = UserRepository::new(pool);

        let unique_id: &str = "test_user_1";
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
        let test_user1 = CreateUserRequest {
            user_id: format!("{}", unique_id),
            user_name: "テストユーザー".to_string(),
            email: format!("{}@example.com", unique_id),
            password_hash: "secret_hash".to_string(),
        };

        let result = repo.create_user(test_user1).await;

        assert!(
            result.is_ok(),
            "ユーザー作成に失敗しました: {:?}",
            result.err()
        );

        let created_user = result.unwrap();
        assert_eq!(created_user.user_name, "テストユーザー");
        assert!(created_user.id > 0);

        // 2. 重複ユーザーIDのテスト
        let test_user2 = CreateUserRequest {
            user_id: format!("{}", unique_id),
            user_name: "テストユーザー2".to_string(),
            email: format!("{}_2@example.com", unique_id),
            password_hash: "secret_hash".to_string(),
        };

        let result2 = repo.create_user(test_user2).await;

        assert!(
            result2.is_err(),
            "重複したuser_idでユーザー作成が成功してしまいました"
        );

        // 3. 重複メールアドレスのテスト
        let test_user3 = CreateUserRequest {
            user_id: format!("{}3", unique_id),
            user_name: "テストユーザー3".to_string(),
            email: format!("{}@example.com", unique_id),
            password_hash: "secret_hash".to_string(),
        };

        let result3 = repo.create_user(test_user3).await;

        assert!(
            result3.is_err(),
            "重複したemailでユーザー作成が成功してしまいました"
        );
    }
}
