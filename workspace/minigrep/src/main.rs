use std::{env, process};

use saka_grep::{run, Config};

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    if let Err(e) = run(&config) {
        eprintln!("Application errored reading file: {e}");
        process::exit(1);
    }
}
