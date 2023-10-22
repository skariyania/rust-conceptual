fn main() {
    println!("hello, now we will panic in another function!");
    _another_scaler();
}
fn _another_scaler() {
    panic!("error occured, false alarm!");
}
