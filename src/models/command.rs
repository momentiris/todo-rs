pub enum Command {
    Add(String),
    Clear,
    List,
    Remove(u32),
    Update(u32),
}

impl Command {
    pub fn new(v: Vec<&str>) -> Option<Self> {
        Self::parse(v)
    }

    fn parse(v: Vec<&str>) -> Option<Command> {
        let command = v.get(0).cloned().unwrap();
        let args = v.get(1).cloned();

        match (command, args) {
            ("add", Some(title)) => Some(Command::Add(title.to_string())),
            ("remove", Some(id)) => Some(Command::Remove(id.parse::<u32>().unwrap())),
            ("list", _) => Some(Command::List),
            ("done", Some(id)) => Some(Command::Update(id.parse::<u32>().unwrap())),
            ("clear", _) => Some(Command::Clear),
            _ => None,
        }
    }
}
