use std::ops::Deref;

use pointer_deref::MyBox;

fn main() {
    let x = 5;
    let y = &x;
    println!("values of (x, y) = ({}, {})", x, y);
    // this is strange, deref, pointers and reference act as same
    println!("values of (y, *y, &y) = ({}, {}, {})", y, *y, &y);
    assert_eq!(5, *y);
    // however below lines wont compile
    // assert_eq!(5, &y);
    // assert_eq!(5, y);

    let z = Box::new(x);
    //deref also works with Box types
    println!("comparing Box values with deref is possible too");
    assert_eq!(x, *z);

    let a = MyBox::new(x);
    println!("value of custom made MyBox: {:?}", a);
    assert_eq!(x, *a);
    assert_eq!(x, *a.deref());
    assert_eq!(&x, a.deref());
}
