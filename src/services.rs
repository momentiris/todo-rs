use crate::db;
use crate::Todo;

pub fn add_todo(todo: Todo) -> Result<Todo, ()> {
    let todos_result = db::get_todos();

    match todos_result {
        Err(_) => Result::Err(()),
        Ok(mut todos) => {
            todos.push(todo.clone());
            let save_result = db::save_todos(todos);

            match save_result {
                Err(_) => Result::Err(()),
                Ok(_) => Result::Ok(todo),
            }
        }
    }
}

pub fn get_todos() -> Result<Vec<Todo>, ()> {
    let result = db::get_todos();

    match result {
        Err(_) => Result::Err(()),
        Ok(todos) => Ok(todos),
    }
}

pub fn remove_todo(todo_id: u32) -> Result<u32, ()> {
    let mut todos = db::get_todos().unwrap();
    todos.retain(|x| x.id != todo_id);

    let result = db::save_todos(todos);
    result.map_or(Err(()), |_| Ok(todo_id))
}
