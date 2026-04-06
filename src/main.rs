// src/main.rs

mod api;
mod auth;
mod db;
mod models;
mod services;
mod router;
mod logging;

use std::sync::Arc;
use tracing::{info, error, debug};

use crate::db::pool::establish_connection;
use crate::db::repository::user::UserRepository;
use crate::services::api_service::ApiService;
use crate::router::create_router;

#[tokio::main]
async fn main() {
    // .envファイルから環境変数を読み込む
    dotenvy::dotenv().ok();
    let _guard=logging::init();

    info!("アプリケーションを起動しています...");
    info!("環境変数を読み込みました。");
    debug!("データベース接続情報: {:?}", std::env::var("DATABASE_URL").unwrap_or_else(|_| "未設定".to_string()));
    // データベース接続プールを確立
    let pool = establish_connection().await;

    info!("データベース接続プールを確立しました。");

    // リポジトリとサービスを初期化
    info!("リポジトリとサービスを初期化しています...");
    let user_repo = UserRepository::new(pool);
    let api_service = ApiService::new(user_repo);
    let shared_state = Arc::new(api_service);

    // ルーターを作成
    info!("apiルーターを作成しています...");
    let app = create_router(shared_state);

    info!("ルーターの作成が完了しました。サーバーを起動します...");
    // サーバーを起動
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("🚀 サーバー起動中: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}