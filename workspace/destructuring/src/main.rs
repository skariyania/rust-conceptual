struct Point {
    x: i32,
    y: i32,
}

//creating macro_rules to return fn name
macro_rules! fn_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

fn main() {
    //asserting struct destructuring values working fine
    struct_destructuring_values();
    struct_destructuring_values_shorthand();
    destructuring_match();
}

fn struct_destructuring_values() {
    println!("@{}", fn_name!());
    // creating point instance and assign reference to 'p'
    let p = Point { x: 0, y: 1 };

    //assign local variable 'a' and 'b' using point ref 'p'
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(1, b);
}

fn struct_destructuring_values_shorthand() {
    println!("@{}", fn_name!());
    // creating point instance and assign reference to 'p'
    let p = Point { x: 0, y: 1 };

    //assign local variable 'a' and 'b' using point ref 'p'
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(1, y);
}

fn destructuring_match() {
    println!("@{}", fn_name!());
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("on the x {x}"),
        Point { x: 0, y } => println!("on the x {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y}")
        }
    }
}
