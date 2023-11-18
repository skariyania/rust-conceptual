// intended function defined in 'c' and invoking in 'rust'
extern "C" {
    //signature of abs function defined in c language
    fn abs(input: i32) -> i32;
}

// intended function defined in 'rust' and invoking in 'c'
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called called a rust function from C");
}

fn main() {
    //1. all foreign function invocations are unsafe in rust
    //2. we shall be able to call foreign language with using extern in rust
    //3. below example demonstrates abs function from 'c' execution in 'rust'
    unsafe {
        println!("Absolute value of -3 according to c: {}", abs(-3));
    }
}
