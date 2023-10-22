#[derive(Debug)]
struct Structure(i32);

// adding struct inside struct
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("printing struct {:#?}", Deep(Structure(2)))
}