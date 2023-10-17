#[derive(Debug)]
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    None,
}
impl Message {
    fn call(&self) {
        println!("enum called");
    }
}
fn main() {
    let home = IPAddrKind::V4(127, 0, 0, 1);
    let loopback = IPAddrKind::V6(String::from("::1"));
    println!("home Address > {:?}", home);
    println!("loop Address > {:?}", loopback);

    let quit_message = Message::Write(String::from("something"));
    Message::call(&quit_message);

    let some_number = Some(5);
    let some_str = Some('d');
    let abs = None;
    let y: Option<i8> = Some(5);
    println!("None type is here  {:?} ", abs);
}
