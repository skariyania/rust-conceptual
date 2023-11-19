//static variables aka. constants
static HOUR_FORMAT: &str = "24";
static mut DEFAULT_TIMEZONE: &str = "IST";

fn change_hour_format() {
    // we can change static variable value within unsafe block
    unsafe { DEFAULT_TIMEZONE = "CET" }
}

fn get_default_timezone<'t>() -> &'t str {
    unsafe { DEFAULT_TIMEZONE }
}
fn main() {
    println!("reading value of global variable: {:?}", HOUR_FORMAT);
    // below statement wont compile as it is not safe to read mutable static variables
    // println!("default timezone: {:?}", DEFAULT_TIMEZONE);

    println!(
        "default timezone before update: {:?}",
        get_default_timezone()
    );
    //changing default timezone is unsafe
    change_hour_format();
    // we can't either access after update mutable static variable without using unsafe
    // as mutable global variable cause data races
    println!(
        "default timezone after update: {:?}",
        get_default_timezone()
    );
}
