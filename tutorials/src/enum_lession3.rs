struct Cat;
struct Dog;
struct Horse;

// enum Pet {
//     Cat(Cat),
//     Dog(Dog),
//     Horse(Horse),
// }

trait Noise {
    fn noise(&self);
}

impl Noise for Cat {
    fn noise(&self) {
        println!("meow");
    }
}

impl Noise for Horse {
    fn noise(&self) {
        println!("honkey");
    }
}
impl Noise for Dog {
    fn noise(&self) {
        println!("bark!");
    }
}
fn main() {
    println!("this works");
    Dog.noise();
}
