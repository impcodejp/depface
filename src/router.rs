use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use crate::handlers::auth::{login_handler, logout_handler};
use crate::handlers::user::register_user_handler;
use crate::services::AppService;

pub fn create_router(state: Arc<AppService>) -> Router {
    let serve_dir =
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

    Router::new()
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/logout", post(logout_handler))
        .route("/api/users", post(register_user_handler))
        .with_state(state)
        .fallback_service(serve_dir)
}
