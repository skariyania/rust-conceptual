use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}
impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("next item in 'a'= {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after mutating a = {}", Rc::strong_count(&b));
    println!("a rc count after mutating a = {}", Rc::strong_count(&a));

    // uncomment next lines to see if we have a cyclic reference
    // this is gonna overflow the stack
    // println!("a next item = {:?}", a.tail());

    // check this visual illustration on cyclic pointers in our scenario
    // case before === println!("a next item = {:?}", a.tail());  ===
    //                     [b]-----↴
    //                             ↓
    //                             ↓
    // [a]-------->[5,_]-------->[10,_]→---↓
    //               ↑                     ↓
    //               ↑                     ↓
    //               ^---------------------←

    //
    // case after === println!("a next item = {:?}", a.tail());  ===
    //
    //            >----[b]------↴
    //            ↑             ↓
    //            ↑     ↓ ------<
    //            ↑     ↓
    //           [a]←---<      Invalid     Invalid
    //
    //               .
    //               cyclic reference [b]<-->[a]
}
