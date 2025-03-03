use axum::{
    routing::{post},
    Router,
};
//use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db::connect, models::Todo, models::CreateTodo};
use sqlx::SqlitePool;
use crate::todo_controller::create;

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/todos", post(create))
        .with_state(pool)
}

