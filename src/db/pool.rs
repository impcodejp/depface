use sqlx::postgres::PgPoolOptions;
use std::env;

pub type DbPool = sqlx::PgPool;

pub async fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}
