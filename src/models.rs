use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub completed: bool,
}
