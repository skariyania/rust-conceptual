#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        ((self.x.powi(2)) + (self.x.powi(2))).sqrt()
    }
}
fn main() {
    let int_map = Point { x: 30, y: 30 };
    let float_map = Point { x: 30.5, y: 40.1 };
    println!(
        "values of int_map {:?}. and value of x,y is {:?},{:?}",
        int_map, int_map.x, int_map.y
    );
    println!("values of float_map {:?}", float_map);

    let floating_distance = Point::distance_from_origin(&float_map);
    println!("floating distance impl {:?}", floating_distance);
}
