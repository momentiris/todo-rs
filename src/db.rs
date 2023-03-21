use crate::config_file::ConfigFile;
use crate::models::todo::Todo;

use std::{fs, io::Write};

use serde_json::{from_str, Result};

pub fn get_todos() -> Result<Vec<Todo>> {
    let data = fs::read_to_string("./todos.json").unwrap();
    let todos: ConfigFile = from_str(&data)?;

    Ok(todos.data)
}

pub fn add_todo(todo: Todo) -> Result<()> {
    let data = fs::read_to_string("./todos.json").unwrap();

    Ok(())
}

pub fn remove_todo(todo_id: u32) -> Result<()> {
    let data = fs::read_to_string("./todos.json").unwrap();
    let todos: ConfigFile = from_str(&data)?;

    Ok(())
}

pub fn init() {
    let file_path = "./todos.json";

    if !fs::metadata(file_path).is_ok() {
        let mut file = fs::File::create(file_path).unwrap();
        file.write_all(b"{\"data\":[]}").unwrap();
        println!("created todo file");
    }
}
