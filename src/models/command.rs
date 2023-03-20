pub enum Command {
    Add(String),
    Remove(String),
    List,
}

impl Command {
    pub fn new(cmd: &str) -> Option<Self> {
        return Self::parse(cmd);
    }

    fn parse(s: &str) -> Option<Command> {
        match s {
            "add" => Some(Command::Add(String::from("..."))),
            "remove" => Some(Command::Remove(String::from("..."))),
            "list" => Some(Command::List),
            _ => None,
        }
    }
}
