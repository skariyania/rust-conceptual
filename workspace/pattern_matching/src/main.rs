fn main() {
    let x = 5;

    //inclusive_range example
    inclusive_range(&x);
    inclusive_range(&50);

    //multiple pattern match example
    multiple_pattern_match(&x);
    multiple_pattern_match(&50);

    //inclusive_range alphabet example
    inclusive_range_abc('c');
    inclusive_range_abc('C');
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

fn inclusive_range_abc(val: char) {
    match val {
        'a'..='j' => println!("ascii value between a to j"),
        'k'..='z' => println!("ascii value between k to z"),
        _ => println!("something else"),
    }
}
