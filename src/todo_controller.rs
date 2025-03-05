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
    let id = Some(Uuid::new_v4().to_string().to_owned());
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
        title: Some(payload.title),
        completed: Some(payload.completed),
    };
    Json(todo)
}

pub async fn get_all(State(pool): State<SqlitePool>) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, completed
        FROM todos
        "#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed fetching todos from the database");

    Json(todos)
}