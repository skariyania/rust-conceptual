fn main() {
    let s1 = String::from("Hello");
    let (s2, _) = string_with_len(s1);
    let length_result = string_with_ref(&s2);
    println!("{s2}, {length_result}");
    let mut s = String::from("Hello 2");
    string_with_mut_ref(&mut s);
    println!("{s}")
}

fn string_with_len(input: String) -> (String, usize) {
    let length = input.len();
    (input, length)
}

fn string_with_ref(input: &String) -> usize {
    let iref = input;
    print!("{iref}");
    input.len()
}
fn string_with_mut_ref(input: &mut String) {
    input.push_str(", string.");
}
