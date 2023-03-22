use serde::{Deserialize, Serialize};

use crate::utils::{get_id, get_timestamp};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub created_at: String,
    pub title: String,
    pub done: bool,
    pub id: u32,
    pub updated_at: String,
}

impl Todo {
    pub fn new(title: String) -> Self {
        let timestamp = get_timestamp();

        return Self {
            created_at: timestamp.to_owned(),
            title: title.to_string(),
            done: false,
            id: get_id(),
            updated_at: timestamp,
        };
    }
}
