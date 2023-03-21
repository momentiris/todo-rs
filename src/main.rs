mod config_file;
mod db;
mod models;
mod utils;

use models::{
    command::Command,
    todo::{self, Todo},
};

use std::io;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = user_input.as_str().trim().split(" ").collect();
    let cmd = Command::new(input);
    db::init();

    match cmd {
        Some(cmd) => handle_command(cmd),
        _ => println!("bad command"),
    }
}

pub fn handle_command(cmd: Command) {
    match cmd {
        Command::List => {
            let todos = db::get_todos().unwrap();
            println!("{:?}", todos);
        }
        Command::Add(v) => db::add_todo(Todo::new(v)).unwrap(),
        Command::Remove(v) => db::remove_todo(v).unwrap(),
    };
}
