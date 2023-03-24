use std::io;

use crate::{
    db::{self, FileTodoRepository, TodoRepository},
    models::{command::Command, todo::Todo},
    services, utils,
};

pub fn start() {
    db::init();
    utils::print_help();

    let r = FileTodoRepository::new("ok".to_string());
    let todo_service = TodoService::new(r);
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = user_input.as_str().trim().split_whitespace().collect();
    let cmd = Command::new(input);

    match cmd {
        Some(cmd) => handle_command(cmd),
        _ => println!("Invalid command..."),
    }
}

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::Add(v) => {
            let result = services::add_todo(Todo::new(v));
            match result {
                Ok(_) => println!("Added todo"),
                Err(_) => println!("Something went wrong when adding todo"),
            }
        }
        Command::List => {
            let result = services::get_todos();
            match result {
                Ok(todos) => utils::print_todos(todos),
                Err(_) => println!("Something went wrong when retrieving todos"),
            }
        }
        Command::Remove(id) => {
            let result = services::remove_todo(id);
            match result {
                Ok(_) => println!("Removed todo {:?}", id),
                Err(_) => println!("Something went wrong when removing todo"),
            }
        }
        Command::Update(id) => {
            let result = services::update_todo(id);
            match result {
                Ok(_) => println!("Updated todo {:?}", id),
                Err(_) => println!("Something went wrong when updating todo"),
            }
        }
        Command::Clear => {
            let result = services::clear_todos();
            match result {
                Ok(_) => println!("Cleared todos"),
                Err(_) => println!("Something went wrong when clearing todos:went"),
            }
        }
    };
}

pub struct TodoService<T: TodoRepository> {
    repository: T,
}

impl<T: TodoRepository> TodoService<T> {
    pub fn new(r: T) -> Self {
        Self { repository: r }
    }
}
