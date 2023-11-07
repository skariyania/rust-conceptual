use std::sync::Mutex;

fn main() {
    mutex_is_simple();
}

fn mutex_is_simple() {
    let m = Mutex::new(3);
    {
        let mut numb = m.lock().unwrap();
        *numb = 6;
    }
    println!("number was updated with mutex lock: {:?}", m);
}
