use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    //send message to channel
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // uncommenting below line with result in compile time error
        // because ownership of val is transferred to transmitter
        // println!("value is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Received from channel {}", received);
}
