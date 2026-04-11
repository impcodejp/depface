// src/services/api_service/auth.rs

use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

use crate::auth::{generate_token, verify_password, verify_token};
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::services::api_service::ApiService;

impl ApiService {
    pub async fn login_user(&self, req: LoginRequest) -> Result<LoginResponse, String> {
        debug!(user_id = %req.user_id, "ユーザー検索開始");
        let user = self
            .user_repo
            .find_by_user_id(&req.user_id)
            .await
            .map_err(|e| format!("DBエラー: {}", e))?
            .ok_or_else(|| {
                warn!(user_id = %req.user_id, "ユーザーが存在しない");
                "ユーザーIDまたはパスワードが正しくありません".to_string()
            })?;

        let is_valid = verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            warn!(user_id = %req.user_id, "パスワード不一致");
            return Err("ユーザーIDまたはパスワードが正しくありません".to_string());
        }

        let token = generate_token(&user.user_id)?;
        debug!(user_id = %user.user_id, "トークン生成完了");

        Ok(LoginResponse {
            token,
            user_id: user.user_id,
        })
    }

    pub async fn logout_user(&self, token: &str) -> Result<(), String> {
        let claims = verify_token(token)?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let ttl = claims.exp as i64 - now;
        if ttl <= 0 {
            // すでに期限切れなので何もしなくてよい
            return Ok(());
        }

        debug!(user_id = %claims.sub, "トークンをブラックリストに登録");
        self.blacklist_repo.blacklist_token(token, ttl).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::hash_password;
    use crate::db::pool::establish_connection;
    use crate::db::repository::user::UserRepository;
    use crate::db::token_blacklist::TokenBlacklistRepository;
    use crate::models::user::CreateUserRequest;
    use dotenvy::dotenv;
    use std::env;

    async fn setup_service() -> ApiService {
        dotenv().ok();
        env::set_var("JWT_SECRET", "test_secret_key_for_testing");

        let pool = establish_connection().await;

        ApiService {
            user_repo: UserRepository::new(pool.clone()),
            blacklist_repo: TokenBlacklistRepository::new(pool),
        }
    }

    async fn create_test_user(service: &ApiService, user_id: &str, password: &str) {
        // テスト前に既存データを削除
        let _ = sqlx::query!("DELETE FROM main.users WHERE user_id = $1", user_id)
            .execute(&service.user_repo.pool)
            .await;

        let password_hash = hash_password(password).unwrap();
        service
            .user_repo
            .create_user(CreateUserRequest {
                user_id: user_id.to_string(),
                user_name: "テストユーザー".to_string(),
                email: format!("{}@example.com", user_id),
                password_hash,
            })
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_login_success() {
        let service = setup_service().await;
        let user_id = "login_test_user";
        let password = "correct_password";
        create_test_user(&service, user_id, password).await;

        let result = service
            .login_user(LoginRequest {
                user_id: user_id.to_string(),
                password: password.to_string(),
            })
            .await;

        assert!(result.is_ok(), "ログインに失敗しました: {:?}", result.err());
        assert_eq!(result.unwrap().user_id, user_id);
    }

    #[tokio::test]
    async fn test_login_wrong_password() {
        let service = setup_service().await;
        let user_id = "login_test_user_wrong_pw";
        create_test_user(&service, user_id, "correct_password").await;

        let result = service
            .login_user(LoginRequest {
                user_id: user_id.to_string(),
                password: "wrong_password".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_nonexistent_user() {
        let service = setup_service().await;

        let result = service
            .login_user(LoginRequest {
                user_id: "no_such_user_xyz".to_string(),
                password: "any_password".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_blacklists_token() {
        let service = setup_service().await;
        let user_id = "logout_test_user";
        let password = "logout_password";
        create_test_user(&service, user_id, password).await;

        // ログインしてトークンを取得
        let login_res = service
            .login_user(LoginRequest {
                user_id: user_id.to_string(),
                password: password.to_string(),
            })
            .await
            .unwrap();

        let token = &login_res.token;

        // ログアウト前はブラックリストに存在しない
        let before = service.blacklist_repo.is_blacklisted(token).await.unwrap();
        assert!(!before, "ログアウト前にトークンがブラックリストに存在しています");

        // ログアウト
        service.logout_user(token).await.unwrap();

        // ログアウト後はブラックリストに存在する
        let after = service.blacklist_repo.is_blacklisted(token).await.unwrap();
        assert!(after, "ログアウト後にトークンがブラックリストに存在していません");
    }
}
