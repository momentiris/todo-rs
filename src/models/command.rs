pub enum Command {
    Add(String),
    Remove(u32),
    List,
}

impl Command {
    pub fn new(v: Vec<&str>) -> Option<Self> {
        return Self::parse(v);
    }

    fn parse(v: Vec<&str>) -> Option<Command> {
        let command = v.get(0).cloned().unwrap();
        let args = v.get(1).cloned();

        match (command, args) {
            ("add", Some(title)) => Some(Command::Add(title.to_string())),
            ("remove", Some(id)) => Some(Command::Remove(id.parse::<u32>().unwrap())),
            ("list", _) => Some(Command::List),
            _ => None,
        }
    }
}
