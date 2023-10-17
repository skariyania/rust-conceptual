fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            println!("none passed");
            None
        }
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six {:?}", six);
    let six_none = plus_one(None);
    println!("six {:?}", six_none);
}
