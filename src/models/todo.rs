use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub created_at: String,
    pub title: String,
    pub done: bool,
    pub id: u32,
    pub updated_at: String,
}
