fn main() {
    let r = returns_closure()(20);
    println!("calling closure with 20 returns {:?}", r);

    let r = returns_closure();
    let r1 = r(40);
    println!("another closure with 40 returns {:?}", r1);
    let r2 = r(50);
    println!("another closure calling again with 50 returns {:?}", r2);
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
