use tracing::{debug, warn};

use crate::auth::hash_password;
use crate::models::user::{
    CreateUserRequest, CreateUserRequestFromFrontend, CreateUserResponseForFrontend,
};
use crate::services::AppService;

impl AppService {
    pub async fn register_user(
        &self,
        frontend_req: CreateUserRequestFromFrontend,
    ) -> Result<CreateUserResponseForFrontend, String> {
        debug!(user_id = %frontend_req.user_id, "パスワードハッシュ化開始");
        let hashed = hash_password(&frontend_req.password)
            .map_err(|e| format!("パスワード暗号化エラー: {}", e))?;

        let db_req = CreateUserRequest {
            user_id: frontend_req.user_id,
            user_name: frontend_req.user_name,
            email: frontend_req.email,
            password_hash: hashed,
        };

        let saved_user = self.user_repo.create_user(db_req).await.map_err(|e| {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("23505") {
                    warn!("重複ユーザー登録試行");
                    return "このIDまたはメールアドレスは既に使用されています".to_string();
                }
            }
            warn!(error = %e, "ユーザー登録DBエラー");
            "ユーザー登録に失敗しました".to_string()
        })?;

        debug!(user_id = %saved_user.user_id, "DBへのユーザー保存完了");
        Ok(CreateUserResponseForFrontend {
            user_id: saved_user.user_id,
            user_name: saved_user.user_name,
            email: saved_user.email,
        })
    }
}
