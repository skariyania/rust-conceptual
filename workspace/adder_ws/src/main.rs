use add_one::{self, add};
use rand;
fn main() {
    let num1 = 3;
    let num2 = 5;
    let add_result = add(num1, num2);
    println!("calling lib crate add and results of add: {add_result}");
}
