fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("First element of vector is : {first}");
    v.push(4);

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetShell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let mut row = vec![
        SpreadsheetShell::Float(5.2),
        SpreadsheetShell::Int(5),
        SpreadsheetShell::Text(String::from("hello")),
    ];
    row.push(SpreadsheetShell::Int(10));

    println!("we have created enum vector {:?}", row);

    let mut string_val = String::from("foo");
    string_val.push_str("bar");
    println!("String can be appended {:?}", string_val + "!");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let format_game = format!("{tic}-{tac}-{toe}");
    println!("we have formed a game: {format_game}");
}
