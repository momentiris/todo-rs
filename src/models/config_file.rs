use serde::{Deserialize, Serialize};

use super::todo::Todo;

#[derive(Serialize, Deserialize, Debug)]

pub struct ConfigFile {
    pub data: Vec<Todo>,
}
