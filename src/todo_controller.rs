use axum::{
    extract::State,
    response::Json,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{Todo, CreateTodo};

pub async fn create(
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