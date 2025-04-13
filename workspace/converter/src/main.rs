use converter::string_to_int;

fn main() {
    let valid_int_1 = "12143".to_string();
    let converted_1 = string_to_int(&valid_int_1);

    if let Some(value) = converted_1 {
        println!("converted value is: {}", value);
    }
    let valid_int_2 = "a12143".to_string();
    let converted_2 = string_to_int(&valid_int_2);

    if let Some(value) = converted_2 {
        println!("converted value is: {}", value);
    }
}
