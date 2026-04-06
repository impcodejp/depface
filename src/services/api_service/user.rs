// src/services/api_service/user.rs

use crate::auth::hash_password;
use crate::models::user::{
    CreateUserRequest, CreateUserRequestFromFrontend, CreateUserResponseForFrontend,
};
use crate::services::api_service::ApiService;

impl ApiService {
    pub async fn register_user(
        &self,
        frontend_req: CreateUserRequestFromFrontend,
    ) -> Result<CreateUserResponseForFrontend, String> {
        let hashed = hash_password(&frontend_req.password)
            .map_err(|e| format!("パスワード暗号化エラー: {}", e))?;

        let db_req = CreateUserRequest {
            user_id: frontend_req.user_id,
            user_name: frontend_req.user_name,
            email: frontend_req.email,
            password_hash: hashed,
        };

        let saved_user = self.user_repo.create_user(db_req).await.map_err(|e| {
            if e.to_string().contains("23505") {
                "このIDまたはメールアドレスは既に使用されています".to_string()
            } else {
                format!("DBエラー: {}", e)
            }
        })?;

        Ok(CreateUserResponseForFrontend {
            user_id: saved_user.user_id,
            user_name: saved_user.user_name,
            email: saved_user.email,
        })
    }
}
