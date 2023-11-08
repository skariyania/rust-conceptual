use std::ops::Deref;

use pointer_deref::{hello, MyBox};

fn main() {
    let x = 5;
    let y = &x;
    // below will print output: (5, 5)
    println!("values of (x, y) = ({}, {})", x, y);
    // this is strange, deref, pointers and reference act as same
    println!("values of (y, *y, &y) = ({}, {}, {})", y, *y, &y); // (5, 5, 5)
    assert_eq!(5, *y);
    // however below lines wont compile
    // assert_eq!(5, &y);
    // assert_eq!(5, y);

    let z = Box::new(x);
    //deref also works with Box types
    println!("comparing Box values with deref is possible too");
    assert_eq!(x, *z);

    let a = MyBox::new(x);
    println!("value of custom made MyBox: {:?}", a); // output: MyBox(5)
    assert_eq!(x, *a);
    assert_eq!(x, *a.deref());
    assert_eq!(&x, a.deref());
    assert_eq!(&x, &*a.deref());
    assert_eq!(&&x, &&*a.deref());
    let m = MyBox::new(String::from("Sahil"));
    println!("deref coercion example &str, &String");
    hello(&m);
    // If Rust didn't implement deref coercion,
    //  we would have to write the code as below
    hello(&(*m));
    //or this is even complex
    //The `(*m)` dereferences the `MyBox<String>` into a String.
    //    Then the `&` and `[..]` take a string slice of the String/
    //    that is equal to the whole string to match the signature of `hello`.
    //    This code without deref coercions is harder to read, write, and understand
    //    with all of these symbols involved. Deref coercion allows Rust to
    //    handle these conversions for us automatically.
    hello(&(*m)[..]);
}
