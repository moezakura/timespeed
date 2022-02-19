use std::env;
use timespeed::domain::config;

fn main() {
    let file_path = get_config_file_path();
    let config_data = config::Config::new(file_path);
    println!("{:?}", config_data);
}

fn get_config_file_path() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return "./config.yaml".to_string();
    }
    let file_path: String = args[1].clone();
    file_path
}
