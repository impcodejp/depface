  use axum::{
      routing::post,
      Router,
  };
  use std::sync::Arc;
  use tower_http::services::{ServeDir, ServeFile};

  use crate::api::auth::{login_handler, logout_handler};
  use crate::api::user::register_user_handler;
  use crate::services::api_service::ApiService;

pub fn create_router(state: Arc<ApiService>) -> Router {
    // React の index.html へのフォールバック設定
    let serve_dir = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));

    Router::new()
        // 認証不要のルート
        .route("/api/auth/login", post(login_handler))
        // 認証必要のルート
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/users", post(register_user_handler))
        .with_state(state)
        .fallback_service(serve_dir)
}
