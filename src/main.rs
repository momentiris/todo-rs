mod app;
mod db;
mod models;
mod services;
mod utils;

use app::app as App;

fn main() {
    App::start()
}
