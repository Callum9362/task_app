use axum::{
    extract::{State},
    response::Json,
    routing::{post},
    Router,
};
//use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db::connect, models::Todo, models::CreateTodo};
use sqlx::SqlitePool;

pub fn create_routes(pool: SqlitePool) -> Router {
    Router::new()
        .route("/todos", post(create_todo))
        .with_state(pool)
}

async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let id = Uuid::new_v4().to_string().to_owned();
    sqlx::query!(
        "INSERT INTO todos(id, title, completed) VALUES (?, ?, ?)",
        id ,
        payload.title,
        payload.completed
    )
    .execute(&pool)
    .await
    .expect("Error inserting into database");

    let todo = Todo {
        id,
        title: payload.title,
        completed: payload.completed,
    };
    Json(todo)
}