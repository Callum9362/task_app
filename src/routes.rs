use axum::{
    routing::{get, post},
    Router,
};
//use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db::connect, models::Todo, models::CreateTodo};
use sqlx::SqlitePool;
use crate::todo_controller::{create, get_all};

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/todos",
            post(create).get(get_all),
        )
        .with_state(pool)
}

