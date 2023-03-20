use std::io;
mod config_file;
mod db;
mod models;
use models::{command::Command, todo};
mod utils;

fn main() {
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = Command::new(user_input.as_str().trim());

    match input {
        Some(cmd) => handle_command(cmd),
        _ => println!("bad command"),
    }
}

pub fn handle_command(cmd: Command) -> () {
    match cmd {
        Command::List => (),
        Command::Add(_) => (),
        Command::Remove(_) => (),
    };
}
