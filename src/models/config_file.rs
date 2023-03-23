use serde::{Deserialize, Serialize};

use crate::todo::Todo;

#[derive(Serialize, Deserialize, Debug)]

pub struct ConfigFile {
    pub data: Vec<Todo>,
}
