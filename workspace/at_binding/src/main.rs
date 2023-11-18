enum Message {
    Hello { id: i32 },
}

fn at_binds(val: Message) {
    match val {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an integer in range 3 to 7: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an integer in range 10 to 12")
        }
        Message::Hello { id } => {
            println!("Found an integer outside of defined range: {}", id)
        }
    }
}

fn main() {
    let msg = Message::Hello { id: 5 };
    at_binds(msg)
}
