use crate::{
    db::{self, FileTodoRepository},
    models::{command::Command, todo::Todo},
    services::todo_service::TodoService,
    utils,
};

pub fn start() {
    let file_path = "./todos.json".to_string();
    db::init(&file_path);
    utils::print_help();

    let todo_repository = FileTodoRepository::new(file_path);
    let todo_service = TodoService::new(todo_repository);
    let user_input = utils::get_input();

    let mut input = user_input.as_str().trim().split_whitespace();
    let cmd = Command::new((input.next(), input.next()));

    match cmd {
        Some(cmd) => handle_command(cmd, todo_service),
        _ => println!("Invalid command..."),
    }
}

pub fn handle_command(cmd: Command, service: TodoService<FileTodoRepository>) {
    match cmd {
        Command::Add(v) => {
            let result = service.add_todo(Todo::new(v));
            match result {
                Ok(_) => println!("Added todo"),
                Err(_) => println!("Something went wrong when adding todo"),
            }
        }
        Command::List => {
            let result = service.get_todos();
            match result {
                Ok(todos) => utils::print_todos(todos),
                Err(_) => println!("Something went wrong when retrieving todos"),
            }
        }
        Command::Remove(id) => {
            let result = service.remove_todo(id);
            match result {
                Ok(_) => println!("Removed todo {:?}", id),
                Err(_) => println!("Something went wrong when removing todo"),
            }
        }
        Command::Update(id) => {
            let result = service.update_todo(id);
            match result {
                Ok(_) => println!("Updated todo {:?}", id),
                Err(_) => println!("Something went wrong when updating todo"),
            }
        }
        Command::Clear => {
            let result = service.clear_todos();
            match result {
                Ok(_) => println!("Cleared todos"),
                Err(_) => println!("Something went wrong when clearing todos:went"),
            }
        }
    };
}
