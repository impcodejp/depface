// src/main.rs

mod api;
mod auth;
mod db;
mod models;
mod services;

use axum::{routing::post, Router};
use std::sync::Arc;
use crate::db::pool::establish_connection;
use crate::db::repository::user::UserRepository;
use crate::services::api_service::ApiService;
use crate::api::user::register_user_handler;

#[tokio::main]
async fn main() {
    // 1. 環境変数とDB接続
    dotenvy::dotenv().ok();
    let pool = establish_connection().await;

    // 2. 依存関係の構築 (Repo -> Service)
    let user_repo = UserRepository::new(pool);
    let api_service = ApiService::new(user_repo);
    
    // 複数のスレッドで共有できるように Arc で包む
    let shared_state = Arc::new(api_service);

    // 3. ルーティング
    let app = Router::new()
        .route("/api/users", post(register_user_handler))
        .with_state(shared_state);

    // 4. サーバー起動
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 サーバー起動中: http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}