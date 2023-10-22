use std::{fs::File, io::ErrorKind};

fn main() {
    let file_path = "/Users/skariyania/src/rust-learnings/tutorials/target/debug/hello.txt";
    let getting_file_result = File::open(file_path);
    let getting_file = match getting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(new_file) => new_file,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => {
                panic!("Problem opening file {:?}", other_error)
            }
        },
    };
    println!("File: {:?}", getting_file);
}
