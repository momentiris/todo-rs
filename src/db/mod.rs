use crate::models::{config_file::ConfigFile, todo::Todo};

use std::{fs, io::Write};

use serde_json::from_str;

pub trait TodoRepository {
    fn save(&self, todos: Vec<Todo>) -> Result<Vec<Todo>, String>;
    fn get(&self) -> Result<Vec<Todo>, String>;
}

pub struct FileTodoRepository {
    file_path: String,
}

impl FileTodoRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl TodoRepository for FileTodoRepository {
    fn save(&self, todos: Vec<Todo>) -> Result<Vec<Todo>, String> {
        let config_file = ConfigFile {
            data: todos.clone(),
        };

        let json_result = serde_json::to_string(&config_file).unwrap();

        let mut file = fs::File::create(&self.file_path).unwrap();
        file.write_all(json_result.as_bytes()).unwrap();

        return Result::Ok(todos);
    }
    fn get(&self) -> Result<Vec<Todo>, String> {
        let data = fs::read_to_string(&self.file_path).unwrap();
        let todos: ConfigFile = from_str(&data).unwrap();

        Ok(todos.data)
    }
}

pub fn init(file_path: &str) {
    if !fs::metadata(file_path).is_ok() {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();
        println!("created todo file");
    }
}
