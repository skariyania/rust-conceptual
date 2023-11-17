struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    Color(Color),
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

    //enum destructuring
    let msg1 = Message::ChangeColor(20, 50, 100);
    message_restructuring(msg1);
    //==
    let msg2 = Message::Quit;
    message_restructuring(msg2);
    //==
    let msg3 = Message::Write("halo".to_string());
    message_restructuring(msg3);
    //==
    let msg3 = Message::Move { x: 1, y: -1 };
    message_restructuring(msg3);
    //==
    let msg4 = Message::Color(Color::Hsv(0, 40, 200));
    message_restructuring(msg4);
    //==
    let msg5 = Message::Color(Color::Rgb(0, 0, 255));
    message_restructuring(msg5);
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
        Point { x, y: 0 } => println!("point is on the x axis {x}"),
        Point { x: 0, y } => println!("point is on the y axis {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y}")
        }
    }
}

fn message_restructuring(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The quit variant in message has no data");
        }
        Message::Move { x, y } => {
            println!("The move variant in message has data: (x={}, y={})", x, y);
        }
        Message::Write(text) => {
            println!("The write variant has text data: '{text}'");
        }
        Message::ChangeColor(r, g, b) => {
            println!(
                "The change color variant has data (r={}, g={}, b={})",
                r, g, b
            );
        }
        Message::Color(Color::Rgb(r, g, b)) => {
            println!(
                "The Color variant has sub variant rgb with data (r={}, g={}, b={})",
                r, g, b
            );
        }
        Message::Color(Color::Hsv(h, s, v)) => {
            println!(
                "The Color variant has sub variant hsv with data (h={}, s={}, v={})",
                h, s, v
            );
        }
    }
}
