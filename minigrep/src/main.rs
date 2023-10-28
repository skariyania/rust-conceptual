use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    config = parse_config(&args);
    println!("Searching for '{}'", query);
    println!("In file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}
