enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!(
        "reference count after creating a = {}",
        Rc::strong_count(&a)
    );
    let _b = Cons(3, Rc::clone(&a));
    println!(
        "reference count after creating b = {}",
        Rc::strong_count(&a)
    );
    {
        let _c = Cons(4, Rc::clone(&a));
        println!(
            "reference count after creating c (local scope) = {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "reference count after creating c (main scope) = {}",
        Rc::strong_count(&a)
    );
}
