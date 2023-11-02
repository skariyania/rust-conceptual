fn main() {
    let x = 5;
    let y = &x;
    println!("values of (x, y) = ({}, {})", x, y);
    // this is wiered, deref, pointers and reference act as same
    println!("values of (y, *y, &y) = ({}, {}, {})", y, *y, &y);
}
