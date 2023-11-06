use std::{clone, sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    let names_tx = tx.clone();

    //send message to channel
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // uncommenting below line with result in compile time error
        // because ownership of val is transferred to transmitter
        // println!("value is {}", val);
    });

    // another example

    //closure fn to send names in channel
    let names_channel = move || {
        let names = vec![
            String::from("Sadi"),
            String::from("George"),
            String::from("Jerry"),
            String::from("Lina"),
        ];

        for val in names {
            names_tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    };

    // names tx thread
    let names_tx_thread = thread::spawn(names_channel);

    // names rx thread
    let names_rx_channel = move || {
        for received_names in rx {
            println!("Howdy, {received_names}");
        }
    };
    thread::spawn(names_rx_channel);

    //block until names thread send all messages
    names_tx_thread.join().unwrap();
}
