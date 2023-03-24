pub enum Command {
    Add(String),
    Clear,
    List,
    Remove(u32),
    Update(u32),
}

type Input<'a> = (Option<&'a str>, Option<&'a str>);

impl Command {
    pub fn new(input: Input) -> Option<Self> {
        Self::parse(input)
    }

    fn parse(input: Input) -> Option<Command> {
        let a = input.0.unwrap_or("");
        let b = input.1.unwrap_or("");

        match (a, b) {
            ("list", _) => Some(Command::List),
            ("clear", _) => Some(Command::Clear),

            ("add", title) => Some(Command::Add(title.to_string())),
            ("remove", id_str) => id_str.parse::<u32>().ok().map(|id| Command::Remove(id)),
            ("done", id_str) => id_str.parse::<u32>().ok().map(|id| Command::Update(id)),
            _ => None,
        }
    }
}
