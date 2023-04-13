use crate::config::Config;

mod config;
mod logger;

fn main() {
    let config = Config::build();
    println!("{config:?}")
}