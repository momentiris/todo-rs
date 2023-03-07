use std::io;

mod todo;

fn main() {
    let mut todos = Vec::<String>::new();
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = todo::Command::new(user_input.as_str().trim());

    match input {
        Some(cmd) => println!("ok"),
        _ => println!("bad command"),
    }
}
