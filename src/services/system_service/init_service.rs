// src/services/system_service/init_service.rs

use crate::models::user::CreateUserRequestFromFrontend;
use crate::services::api_service::ApiService;

pub async fn user_check_and_first_user_registration(api_service: &ApiService) {
    let user_count = api_service.user_repo.count_users().await.unwrap_or(0);

    if user_count == 0 {
        println!("ユーザーが存在しません。初期ユーザーを登録します...");
        let adminuser = CreateUserRequestFromFrontend {
            user_id: "mjscs".to_string(),
            user_name: "MjsAdmin".to_string(),
            email: "admin@example.com".to_string(),
            password: "MJS369CS".to_string(),
        };
        let _ = api_service.register_user(adminuser).await;
    }
}
