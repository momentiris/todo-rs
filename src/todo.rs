#[derive(Debug)]

enum CommandType {
    Add(String),
    Remove(String),
    List,
}

#[derive(Debug)]
pub struct Command(CommandType);

impl Command {
    pub fn new(cmd: &str) -> Option<Self> {
        return Self::parse(cmd);
    }

    fn parse(s: &str) -> Option<Command> {
        match s {
            "add" => Some(Command(CommandType::Add(String::from("...")))),
            "remove" => Some(Command(CommandType::Remove(String::from("...")))),
            "list" => Some(Command(CommandType::Add(String::from("...")))),
            _ => None,
        }
    }
}

pub fn handle_command(cmd: Command) -> () {
    match cmd {
        Command(CommandType::List) => (),
        Command(CommandType::Add(_)) => (),
        Command(CommandType::Remove(_)) => (),
    };
}
