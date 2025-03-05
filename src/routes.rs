use axum::{
    routing::{get, post},
    Router,
};
//use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db::connect, models::Todo, models::CreateTodo};
use sqlx::SqlitePool;
use crate::todo_controller::{create, get_all, get_by_id};

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/todos",
            post(create).get(get_all),
        )
        .route(
            "/todos/:id",
            get(get_by_id),
        )
        .with_state(pool)
}

