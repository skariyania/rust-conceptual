use std::slice;

// we can use unsafe keyword on function definition
// this is an example of how to create unsafe function in rust
// also we must call unsafe function within separate unsafe block
unsafe fn dangerous() {}

/*
we can also have safe function and unsafe implementation within it
this is ideal way to wrap unsafe code abstracted in safe function
also implementing this unsafe this way minimizes amount of unsafe code
*/
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])
    // above line wont compile, hence we are creating unsafe block below
    // to achieve same functionality
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    // NOTE: we can create raw pointers in safe code, we just can't
    // deference raw pointers outside unsafe block
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // this statement will print out r1 and r2 memory addresses
    println!("values of references r1 = {:?}, r2 = {:?}", r1, r2);

    //accessing data from random memory location
    let addr = 0x16b5763b4usize;
    let r = addr as *const i32;
    println!("accessing memory location at random: {:?}", r);
    // although we can access memory address, it is not
    // safe to access data at this location
    // hence if we try to access data, we will get compile time error
    // below line will not compile
    // println!("accessing memory location at random: {:?}", r);

    //so to access data, we will now use unsafe block
    //however it is not guaranteed that program will work correctly
    unsafe {
        println!("accessing memory location at r1: {:?}", *r1);
        println!("accessing memory location at r2: {:?}", *r2);
        // below line will compile however not guaranty or its running a program
        // println!("accessing memory location at random: {:?}", *r);
        dangerous()
    }
}
