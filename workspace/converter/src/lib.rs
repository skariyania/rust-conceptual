pub fn string_to_int(input: &str) -> Option<i32> {
    match input.parse() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Error: Unable to convert string to integer");
            None
        }
    }
}
