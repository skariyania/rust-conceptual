use std::{fs, io};

fn main() {
    fn read_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.st")
    }
    println!("{:?}", read_file());
}
