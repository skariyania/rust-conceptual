struct Point {
    x: i32,
    y: i32,
}
fn main() {
    //asserting struct destructuring values working fine
    struct_destructuring_values();
}

fn struct_destructuring_values() {
    // creating point instance and assign reference to 'p'
    let p = Point { x: 0, y: 1 };

    //assign local variable 'a' and 'b' using point ref 'p'
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(1, b);
}
