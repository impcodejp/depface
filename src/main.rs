// src/main.rs

mod api;
mod auth;
mod db;
mod logging;
mod middleware;
mod models;
mod router;
mod services;

use std::sync::Arc;
use tracing::info;

use crate::db::repository::user::UserRepository;
use crate::db::token_blacklist::TokenBlacklistRepository;
use crate::router::create_router;
use crate::services::api_service::ApiService;
use crate::services::system_service::init_service::user_check_and_first_user_registration;

#[tokio::main]
async fn main() {
    // 1. 初期設定（環境変数・ログ）
    dotenvy::dotenv().ok();
    let _guard = logging::init();

    info!("アプリケーションを起動しています...");

    // 2. データベース（PostgreSQL）接続確立
    let pool = db::establish_connection().await;
    info!("データベース接続プールを確立しました。");

    // 3. リポジトリとサービスの初期化
    let user_repo = UserRepository::new(pool.clone());
    let blacklist_repo = TokenBlacklistRepository::new(pool);
    let api_service = ApiService::new(user_repo, blacklist_repo);

    // 共有状態（State）として Arc に包む
    let shared_state = Arc::new(api_service);

    // ユーザー登録確認
    user_check_and_first_user_registration(&shared_state).await;

    // 4. ルーターの作成とサーバー起動
    info!("apiルーターを作成しています...");
    let app = create_router(shared_state);

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    info!("サーバー起動中: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
