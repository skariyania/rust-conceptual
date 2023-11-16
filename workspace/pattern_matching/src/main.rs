fn main() {
    let x = 5;
    inclusive_range(&x);
    inclusive_range(&50);
    multiple_pattern_match(&x);
    multiple_pattern_match(&50);
}

fn inclusive_range(val: &u32) {
    match val {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

fn multiple_pattern_match(val: &u32) {
    match val {
        1 | 2 | 3 | 4 | 5 => println!("multiple pattern match 1 to 5"),
        _ => println!("anything else in multiple pattern"),
    }
}
