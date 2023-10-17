fn main() {
    let mut s = String::from("Hello");
    let r2 = &mut s;
    println!("{}", r2);
    let r3 = &mut s;
    println!("{}", r3);
}
