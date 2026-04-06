// src/services/api_service.rs// src/services/api_service.rs

use crate::db::repository::user::UserRepository;
use crate::models::user::{CreateUserRequest, 
    CreateUserRequestFromFrontend, 
    CreateUserResponseForFrontend
};
use crate::auth::{verify_password, generate_token};
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::auth::hash_password;

pub struct ApiService {
    user_repo: UserRepository,
}

impl ApiService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(
        &self, 
        frontend_req: CreateUserRequestFromFrontend
    ) -> Result<CreateUserResponseForFrontend, String> {
        // 1. 生パスワードをハッシュ化
        let hashed = hash_password(&frontend_req.password)
            .map_err(|e| format!("パスワード暗号化エラー: {}", e))?;

        // 2. DB用のリクエストに詰め替え
        let db_req = CreateUserRequest {
            user_id: frontend_req.user_id,
            user_name: frontend_req.user_name,
            email: frontend_req.email,
            password_hash: hashed,
        };

        // 3. DBへ保存
        let saved_user = self.user_repo
            .create_user(db_req)
            .await
            .map_err(|e| {
                if e.to_string().contains("23505") {
                    "このIDまたはメールアドレスは既に使用されています".to_string()
                } else {
                    format!("DBエラー: {}", e)
                }
            })?;

        // 4. 保存された User (Entity) を フロント用レスポンスに変換して返す
        Ok(CreateUserResponseForFrontend {
            user_id: saved_user.user_id,
            user_name: saved_user.user_name,
            email: saved_user.email,
        })
    }

    // ログイン処理
    pub async fn login_user(
    &self,
    req: LoginRequest,
) -> Result<LoginResponse, String> {
    // 1. user_idでユーザーを検索
    let user = self.user_repo
        .find_by_user_id(&req.user_id)
        .await
        .map_err(|e| format!("DBエラー: {}", e))?
        .ok_or_else(|| "ユーザーIDまたはパスワードが正しくありません".to_string())?;

    // 2. パスワード検証
    let is_valid = verify_password(&req.password, &user.password_hash)?;
    if !is_valid {
        return Err("ユーザーIDまたはパスワードが正しくありません".to_string());
    }

    // 3. JWTを生成
    let token = generate_token(&user.user_id)?;

    Ok(LoginResponse {
        token,
        user_id: user.user_id,
    })
}
}