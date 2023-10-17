// struct Point {
//     x: f64,
//     y: f64,
// }
// impl Point {
//     fn origin() -> Point {
//         Point { x: 0.0, y: 0.0 }
//     }

//     fn new(x: f64, y: f64) -> Point {
//         Point { x: x, y: y }
//     }
// }

fn main() {
    // let p1 = Point::origin();
    // println!("{} {}", p1.x, p1.y);

    // let p2 = Point::new(1.0, 2.0);
    // print!("{} {}", p2.x, p2.y);
    // let x = 2;
    // let x = x + 35;
    // println!("printing {x}");
    another_func(2, "2".to_string());
}

fn another_func(x: i32, y: String) {
    println!("printing x {x} {y}");
}
