use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("\nPlease input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("type valid number!");
                continue;
            }
        };
        println!("The secret number is : {secret_number}");
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed too small!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed correct.");
                break;
            }
            Ordering::Greater => println!("You guessed too big!"),
        }
    }
}
