use colorize::*;
pub(crate) use rand::prelude::*;

pub fn get_timestamp() -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%m-%d %H:%M").to_string();

    timestamp
}

pub fn get_id() -> u32 {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen_range(1..1000);

    id + rng.gen_range(1..1000)
}

pub fn print_help() {
    let help = format!(
        "
            {} {}
            {}
            -----
        
            Help:
        
            Command   | Arguments | Description
            {}           text        Add a new todo
            {}                       List all todos
            {}           id          Mark a todo as done
            {}           id          Delete a todo
        ",
        "Welcome to".grey(),
        "TodoBook".cyan(),
        "Simple todo app written in Rust".black(),
        "add".cyan(),
        "list".blue(),
        "done".green(),
        "remove".red()
    );

    println!("{help}");
}
