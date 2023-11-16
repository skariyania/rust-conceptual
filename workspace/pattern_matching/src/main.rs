fn main() {
    let x = 5;
    inclusive_range(&x);
    inclusive_range(&50);
}

fn inclusive_range(val: &u32) {
    match val {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
