use std::env;

pub mod game;
pub mod config;

use config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    game::run(parse_config(&args));
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 4 {
        panic!("Not enough arguments");
    }
    let (w, h) = (
        args[1].parse::<i32>().expect("Expected a number for the width"),
        args[2].parse::<i32>().expect("Expected a number for the height")
        );
    let title = &args[3];
    let config = Config::new(w, h, title.to_string());
    return config;
}

