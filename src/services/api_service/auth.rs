// src/services/api_service/auth.rs

use crate::services::api_service::ApiService;
use crate::auth::{verify_password, generate_token};
use crate::models::auth::{LoginRequest, LoginResponse};

impl ApiService {
    pub async fn login_user(
        &self,
        req: LoginRequest,
    ) -> Result<LoginResponse, String> {
        let user = self.user_repo
            .find_by_user_id(&req.user_id)
            .await
            .map_err(|e| format!("DBエラー: {}", e))?
            .ok_or_else(|| "ユーザーIDまたはパスワードが正しくありません".to_string())?;

        let is_valid = verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err("ユーザーIDまたはパスワードが正しくありません".to_string());
        }

        let token = generate_token(&user.user_id)?;

        Ok(LoginResponse {
            token,
            user_id: user.user_id,
        })
    }
    
    // 今後ここに logout メソッドを追加します
}