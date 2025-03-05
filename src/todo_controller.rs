use axum::{
    extract::State,
    response::Json,
};
use axum::extract::Path;
use axum::http::StatusCode;
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

pub async fn get_by_id(
    Path(id): Path<String>,
    State(pool): axum::extract::State<SqlitePool>,
) -> Result<Json<Todo>, StatusCode> {

    let result = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = ?",
        id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(todo) => Ok(Json(todo)),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use sqlx::{SqlitePool, Executor};

    // Tokio is required for running async tests
    #[tokio::test]
    async fn test_get_all() {
        // Arrange: Set up an in-memory SQLite database
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Create the `todos` table
        pool.execute(
            r#"
            CREATE TABLE todos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            );
            "#
        )
            .await
            .unwrap();

        // Insert example data
        pool.execute(
            r#"
            INSERT INTO todos (id, title, completed)
            VALUES
            ("1", "Test Todo 1", false),
            ("2", "Test Todo 2", true);
            "#
        )
            .await
            .unwrap();

        // Act: Call the `get_all` function
        let response = get_all(State(pool.clone())).await;

        // Assert: Verify the response
        if let Json(todos) = response {
            assert_eq!(todos.len(), 2); // Confirm the number of todos
            assert_eq!(todos[0].id.as_deref(), Some("1")); // Confirm ID of the first todo
            assert_eq!(todos[0].title.as_deref(), Some("Test Todo 1")); // Confirm title of the first todo
            assert_eq!(todos[1].completed, Some(true)); // Confirm completion status of the second todo
        } else {
            panic!("Response was not Json<Vec<Todo>>");
        }
    }

    #[tokio::test]
    async fn test_get_by_id() {
        // Arrange: Set up an in-memory SQLite database
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        // Create the `todos` table
        pool.execute(
            r#"
            CREATE TABLE todos (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            );
            "#
        )
            .await
            .unwrap();

        // Insert example data
        pool.execute(
            r#"
            INSERT INTO todos (id, title, completed)
            VALUES
            ("1", "Test Todo 1", false),
            ("2", "Test Todo 2", true);
            "#
        )
            .await
            .unwrap();

        // Act: Call the `get_all` function
        let id = "1".to_string();
        let response = get_by_id(Path(id), State(pool.clone()))
            .await
            .unwrap();

        assert_eq!(response.title.as_deref(), Some("Test Todo 1"));
    }

    #[tokio::test]
    async fn test_create() {
        // Arrange
        let pool = SqlitePool::connect(":memory:").await.unwrap();

        pool.execute(
            r#"
        CREATE TABLE todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        );
        "#
        )
            .await
            .unwrap();

        let new_todo = CreateTodo {
            title: "Learn Rust".to_string(),
            completed: false,
        };

        // Act
        let todo: Json<Todo> = create(State(pool.clone()), Json(new_todo)).await;

        // Assert
        assert_eq!(todo.0.title.as_deref(), Some("Learn Rust"));
    }


}
