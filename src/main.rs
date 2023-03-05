use std::io;

#[derive(Debug)]
pub struct TodoCommand(String);

impl TodoCommand {
    pub fn new(cmd: &str) -> Option<Self> {
        return Self::parse(cmd);
    }

    fn parse(s: &str) -> Option<TodoCommand> {
        match s {
            "add" | "remove" | "list" => Some(TodoCommand(s.to_string())),
            _ => None,
        }
    }
}

fn main() {
    let mut tods = Vec::<String>::new();
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input).expect("input failure");

    let input = TodoCommand::new(user_input.as_str().trim());

    match input {
        Some(cmd) => println!("ok"),
        _ => println!("bad command"),
    }
}
