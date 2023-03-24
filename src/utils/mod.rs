use std::io;

use colorize::*;
pub(crate) use rand::prelude::*;

use crate::models::todo::Todo;

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%m-%d %H:%M").to_string();

    timestamp
}

pub fn get_id() -> u32 {
    let mut rng = rand::thread_rng();

    rng.gen_range(1..1000) + rng.gen_range(1..1000)
}

pub fn print_help() {
    let help = format!(
        "
            {}
            {}
            -----
        
            Help:
        
            Command   |  Arguments  |    Description
            {}            text          Add a new todo
            {}            id           Mark a todo as done
            {}          id           Delete a todo
            {}                        Delete all todos
            {}                         List all todos
        ",
        "Welcome to".grey(),
        "A Simple Todo App ™️".cyan(),
        "add".cyan(),
        "done".green(),
        "remove".red(),
        "clear".green(),
        "list".blue(),
    );

    println!("{help}");
}

pub fn print_todos(todos: Vec<Todo>) {
    println!(
        "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
        "ID", "Title", "Created at", "Updated at", "Done"
    );

    todos.iter().for_each(|todo| {
        println!(
            "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            todo.id,
            todo.title,
            todo.created_at,
            todo.updated_at,
            if todo.done {
                "Completed 😸".green()
            } else {
                "No 😿".red()
            }
        )
    })
}

pub fn get_input() -> String {
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");
    user_input
}
