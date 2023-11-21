fn main() {
    let list_of_numbers = vec![1, 2, 3];

    //converts list of numbers to list of strings
    // using closure
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("List of strings using closure: {:?}", list_of_strings);

    // using function
    let list_of_strings_f: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("List of strings using function: {:?}", list_of_strings_f);
}
