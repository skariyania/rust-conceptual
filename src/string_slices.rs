fn main() {
    let s = String::from("string of words");
    let s2 = first_world(&s);
    println!("out ==> {}", s2);
}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
