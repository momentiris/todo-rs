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
