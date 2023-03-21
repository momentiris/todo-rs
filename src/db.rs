use crate::config_file::ConfigFile;
use crate::models::todo::Todo;

use std::{fs, io::Write};

use serde_json::{from_str, Result};

pub fn get_todos() -> Result<Vec<Todo>> {
    let data = fs::read_to_string("./todos.json").unwrap();
    let todos: ConfigFile = from_str(&data)?;

    Ok(todos.data)
}

pub fn save_todos(todos: Vec<Todo>) -> Result<()> {
    let config_file = ConfigFile { data: todos };
    let json = serde_json::to_string(&config_file).unwrap();

    let mut file = fs::File::create("./todos.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();

    Ok(())
}

pub fn add_todo(todo: Todo) -> Result<()> {
    let mut todos = get_todos().unwrap();
    todos.push(todo);

    save_todos(todos).unwrap();
    Ok(())
}

pub fn remove_todo(todo_id: u32) -> Result<()> {
    let mut todos = get_todos().unwrap();
    todos.retain(|x| x.id != todo_id);
    save_todos(todos).unwrap();

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
