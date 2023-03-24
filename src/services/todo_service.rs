use crate::{db::TodoRepository, models::todo::Todo, utils};

pub struct TodoService<T: TodoRepository> {
    repository: T,
}

impl<T: TodoRepository> TodoService<T> {
    pub fn new(r: T) -> Self {
        Self { repository: r }
    }

    pub fn add_todo(&self, todo: Todo) -> Result<Todo, ()> {
        match self.repository.get() {
            Err(_) => Result::Err(()),
            Ok(mut todos) => {
                todos.push(todo.clone());
                self.repository.save(todos).map_or(Err(()), |_| Ok(todo))
            }
        }
    }

    pub fn clear_todos(&self) -> Result<(), ()> {
        match self.repository.get() {
            Err(_) => Err(()),
            Ok(mut todos) => {
                todos.clear();
                self.repository
                    .save(todos)
                    .map_or_else(|_| Err(()), |_| Ok(()))
            }
        }
    }

    pub fn get_todos(&self) -> Result<Vec<Todo>, ()> {
        self.repository.get().map_or(Err(()), |todos| Ok(todos))
    }

    pub fn remove_todo(&self, todo_id: u32) -> Result<u32, ()> {
        match self.repository.get() {
            Err(_) => Err(()),
            Ok(mut todos) => {
                todos.retain(|x| x.id != todo_id);

                self.repository
                    .save(todos)
                    .map_or_else(|_| Err(()), |_| Ok(todo_id))
            }
        }
    }

    pub fn update_todo(&self, todo_id: u32) -> Result<Todo, ()> {
        match self.repository.get() {
            Err(_) => Result::Err(()),
            Ok(mut todos) => {
                let target_index = todos.iter().position(|todo| todo.id == todo_id).unwrap();

                todos[target_index].done = true;
                todos[target_index].updated_at = utils::get_timestamp();

                let updated_todo = todos[target_index].clone();
                self.repository
                    .save(todos)
                    .map_or_else(|_| Err(()), |_| Ok(updated_todo))
            }
        }
    }
}
