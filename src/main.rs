use std::io;

mod todo;

type Todo = String;

fn main() {
    let mut todos = Vec::<Todo>::new();
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = todo::Command::new(user_input.as_str().trim());

    match input {
        Some(cmd) => handle_command(cmd, todos),
        _ => println!("bad command"),
    }
}

pub fn handle_command(cmd: todo::Command, state: Vec<Todo>) -> () {
    match cmd {
        todo::Command::List => (),
        todo::Command::Add(_) => (),
        todo::Command::Remove(_) => (),
    };
}
