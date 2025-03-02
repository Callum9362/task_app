use sqlx::{SqlitePool, Pool, Sqlite};
use std::env;

pub async fn connect() -> Pool<Sqlite> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://todos.db".into());
    SqlitePool::connect(&db_url).await.expect("Failed to connect to database")
}