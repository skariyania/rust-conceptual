#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 5,
        Coin::Quarter(state) => {
            println!("coin state: {:?}", state);
            25
        }
    }
}

fn main() {
    let coin_inst = Coin::Quarter(UsState::Alaska);
    println!("calling {:?}", value_in_cents(coin_inst));
}
