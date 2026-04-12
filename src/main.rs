mod auth;
mod db;
mod handlers;
mod logging;
mod middleware;
mod models;
mod router;
mod services;

use std::sync::Arc;
use tracing::info;

use crate::db::token_blacklist::TokenBlacklistRepository;
use crate::db::user::UserRepository;
use crate::models::user::CreateUserRequestFromFrontend;
use crate::router::create_router;
use crate::services::AppService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let _guard = logging::init();

    info!("アプリケーションを起動しています...");

    let pool = db::establish_connection().await;
    info!("データベース接続プールを確立しました。");

    let user_repo = UserRepository::new(pool.clone());
    let blacklist_repo = TokenBlacklistRepository::new(pool);
    let service = AppService::new(user_repo, blacklist_repo);

    seed_initial_user(&service).await;

    let shared_state = Arc::new(service);

    info!("apiルーターを作成しています...");
    let app = create_router(shared_state);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    info!("サーバー起動中: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

async fn seed_initial_user(service: &AppService) {
    let user_count = service.user_repo.count_users().await.unwrap_or(0);
    if user_count == 0 {
        info!("ユーザーが存在しません。初期ユーザーを登録します...");
        let admin = CreateUserRequestFromFrontend {
            user_id: "mjscs".to_string(),
            user_name: "MjsAdmin".to_string(),
            email: "admin@example.com".to_string(),
            password: "MJS369CS".to_string(),
        };
        let _ = service.register_user(admin).await;
    }
}
