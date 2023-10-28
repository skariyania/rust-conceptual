use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    match config {
        Ok(config) => {
            println!("Searching for '{}'", config.query);
            println!("Searching for '{}'", config.query);
            let contents = fs::read_to_string(&config.file_path)
                .expect("Should have been able to read the file");

            println!("With text:\n{contents}");
            println!("In file '{}'", &config.file_path);
        }
        Err(message) => {
            println!("{}", message);
        }
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
