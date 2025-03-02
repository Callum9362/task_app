use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub completed: bool,
}
