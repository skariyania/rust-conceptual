use tutorials::front_of_house::hosting::add_to_waitlist;

fn main() {
    let config_max = Some(4u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     None => println!("None found"),
    // }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
    add_to_waitlist();
}
