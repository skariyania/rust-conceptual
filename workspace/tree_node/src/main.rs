use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    println!("Hello, world!");
}
