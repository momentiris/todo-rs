mod db;
mod models;
mod services;
mod utils;

use models::{
    command::Command,
    todo::{self, Todo},
};

use std::io;

fn main() {
    db::init();
    utils::print_help();

    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = user_input.as_str().trim().split(" ").collect();

    let cmd = Command::new(input);
    match cmd {
        Some(cmd) => handle_command(cmd),
        _ => println!("bad command"),
    }
}

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::List => {
            let result = services::get_todos();
            match result {
                Ok(todos) => println!("{:?}", todos),
                Err(_) => println!("Something went wrong when retrieving todos"),
            }
        }
        Command::Remove(v) => {
            let result = services::remove_todo(v);
            match result {
                Ok(_) => println!("Removed todo {:?}", v),
                Err(_) => println!("Something went wrong when removing todo"),
            }
        }
        Command::Add(v) => {
            let result = services::add_todo(Todo::new(v));
            match result {
                Ok(_) => println!("Added todo"),
                Err(_) => println!("Something went wrong when adding todo"),
            }
        }
    };
}
