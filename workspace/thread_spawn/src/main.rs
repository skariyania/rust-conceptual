use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!(
                "I am thread id: {i} and I have captured vec from main: {:?}",
                v
            );
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("number from main thread is: {}", i);
    }

    // use handle join to wait for all the thread to complete processing
    // note: calling join blocks the thread until all handle thread completes
    // this also prevents other operations on main
    handle.join().unwrap();

    // this print statement is blocked by handle.join()
    println!("main will now exit");
}
