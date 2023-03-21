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
        let command = v.get(0).cloned();
        let args = v.get(1).cloned();

        println!("{:?}", command);
        println!("{:?}", args);
        match (command, args) {
            (Some("add"), Some(title)) => Some(Command::Add(String::from(title.clone()))),
            (Some("remove"), Some(id)) => Some(Command::Remove(id.parse::<u32>().unwrap())),
            (Some("list"), _) => Some(Command::List),
            _ => None,
        }
    }
}
