struct Person {
    name: String,
    height: u32,
}
fn main() {
    let charlie = Person {
        name: "charlie".to_sting(),
        height: 54,
    };
    println!("{:?}", charlie.name);
    dbg!(charlie.name);
}
