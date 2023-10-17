#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 40,
    };
    let rect2 = Rectangle {
        height: 20,
        width: 10,
    };
    let area1 = rect1.area();
    let area2 = rect2.area();
    println!("area of rect1 {:#?}", area1);
    println!("area of rect2 {:#?}", area2);
    println!("can rect1 hold rect2? {:?}", rect1.can_hold(&rect2));

    let square_instance = Rectangle::square(30);
    println!("{:?}", square_instance);
}
