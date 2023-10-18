fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2];

    let mut v3 = Vec::new();
    v3.push(2);
    v3.push(3);
    println!("vector v3 value {:?}", v3);

    let mut v4 = vec![];
    v4.push(1);

    let third = &v4[0];
    println!("value of first element of v4 {third}");
}
