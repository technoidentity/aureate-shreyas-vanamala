#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

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
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    let coins = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
    ];

    for coin in coins {
        println!("Coin value: {} cents", value_in_cents(coin));
    }
}
