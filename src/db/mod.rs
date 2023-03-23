use crate::models::{config_file::ConfigFile, todo::Todo};

use std::{fs, io::Write};

use serde_json::{from_str, Result as SerdeResult};

pub fn save_todos(todos: Vec<Todo>) -> SerdeResult<Vec<Todo>> {
    let config_file = ConfigFile {
        data: todos.clone(),
    };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create("./todos.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    return Result::Ok(todos);
}

pub fn get_todos() -> SerdeResult<Vec<Todo>> {
    let data = fs::read_to_string("./todos.json").unwrap();
    let todos: ConfigFile = from_str(&data)?;

    Ok(todos.data)
}

pub fn init() {
    let file_path = "./todos.json";

    if !fs::metadata(file_path).is_ok() {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();
        println!("created todo file");
    }
}
