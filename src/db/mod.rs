pub mod pool;
pub mod repository;

// 外部から「db::establish_connection」で呼べるように再公開
pub use pool::establish_connection;
pub use pool::DbPool;