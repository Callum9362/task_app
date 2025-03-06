use axum::{
    routing::{get, post},
    Router,
};

use sqlx::SqlitePool;
use crate::todo_controller::{create, get_all, get_by_id, update};

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route(
            "/todos",
            post(create).get(get_all),
        )
        .route(
            "/todos/:id",
            get(get_by_id).put(update),
        )
        .with_state(pool)
}

