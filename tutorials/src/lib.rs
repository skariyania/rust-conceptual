use std::thread;

pub fn public_function() {
    println!("hello from lib");
    another_private_function();
}

fn another_private_function() {
    println!("printed with another private fn inside library");
}

pub mod front_of_house;
pub mod lifetimes;
pub mod media_aggrigator;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}

pub fn threading_example() {
    let list = vec![1, 2, 3];
    println!("before defining closure list values: {:?}", list);
    thread::spawn(move || println!("from thread, list values: {:?}", list))
        .join()
        .unwrap();
}
