use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    mutex_is_simple();
    mutex_with_arc();
}

fn mutex_with_arc() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut numb = counter.lock().unwrap();

            *numb += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!(
        "Result after all thread completes: {:?}",
        *counter.lock().unwrap()
    );
}

fn mutex_is_simple() {
    let m = Mutex::new(3);

    {
        let mut numb = m.lock().unwrap();
        *numb = 6;
    }

    println!("number was updated with mutex lock: {:?}", m);
}
