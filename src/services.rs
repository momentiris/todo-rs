use crate::db;
use crate::Todo;

pub fn add_todo(todo: Todo) -> Result<Todo, ()> {
    match db::get_todos() {
        Err(_) => Result::Err(()),
        Ok(mut todos) => {
            todos.push(todo.clone());
            db::save_todos(todos).map_or(Err(()), |_| Ok(todo))
        }
    }
}

pub fn get_todos() -> Result<Vec<Todo>, ()> {
    db::get_todos().map_or(Err(()), |todos| Ok(todos))
}

pub fn remove_todo(todo_id: u32) -> Result<u32, ()> {
    let mut todos = db::get_todos().unwrap();
    todos.retain(|x| x.id != todo_id);

    db::save_todos(todos).map_or(Err(()), |_| Ok(todo_id))
}

pub fn update_todo(todo_id: u32) -> Result<Todo, ()> {
    match db::get_todos() {
        Err(_) => Result::Err(()),
        Ok(mut todos) => {
            let target_index = todos.iter().position(|todo| todo.id == todo_id).unwrap();
            todos[target_index].done = true;

            let updated_todo = todos[target_index].clone();
            db::save_todos(todos).map_or(Err(()), |_| Ok(updated_todo))
        }
    }
}
