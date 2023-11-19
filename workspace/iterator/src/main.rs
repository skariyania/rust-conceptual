use iterator::Point;

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let point_add_result = points_add(p1, p2);
    println!(
        "result of adding two points: \n  p1={:?} \n+ p2={:?}\n=    {:?}",
        p1, p2, point_add_result
    );
}

fn points_add(p1: Point, p2: Point) -> Point {
    p1 + p2
}
