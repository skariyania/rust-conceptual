fn main() {
    let list_of_numbers = vec![1, 2, 3];

    //converts list of numbers to list of strings
    // using closure
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("List of strings using closure: {:?}", list_of_strings);

    // using function
    let list_of_strings_f: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("List of strings using function: {:?}", list_of_strings_f);

    //Note: closure and function as parameter both compiles to same code

    //we can also use enums in map
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0..5).map(Status::Value).collect();
    println!("collected enum values {:?}", list_of_statuses);
}
