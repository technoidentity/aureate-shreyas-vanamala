use std::cmp::Ordering;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn route(address: IpAddr) {
    match address {
        IpAddr::V4(value) => println!("IPv4: {value}"),
        IpAddr::V6(value) => println!("IPv6: {value}"),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Message: {text}"),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to ({r}, {g}, {b})");
            }
        }
    }
}

fn describe(value: Option<i32>) {
    let Some(number) = value else {
        println!("No number was provided");
        return;
    };

    println!("The number is {number}");
}

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(number) => Some(number + 1),
    }
}

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
    // Structs, methods, and associated functions
    let square = Rectangle::square(20);
    let small_rectangle = Rectangle {
        width: 10,
        height: 15,
    };

    println!("Square area: {}", square.area());
    println!(
        "Can hold the smaller rectangle: {}",
        square.can_hold(&small_rectangle)
    );

    // Enum variants
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Address kinds: {four:?}, {six:?}");

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    // Enums carrying different kinds of data
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for message in &messages {
        message.call();
    }

    // Option and let-else
    describe(Some(10));
    describe(None);

    // if-let
    let config_max = Some(3_u8);

    if let Some(max) = config_max {
        println!("The maximum is {max}");
    } else {
        println!("No maximum value was set");
    }

    // Match bindings and wildcard patterns
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Move three spaces"),
        7 => println!("Collect a reward"),
        other => println!("Move {other} spaces"),
    }

    match dice_roll {
        3 => println!("Special action"),
        _ => (),
    }

    // Returning Option values
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("Five plus one: {six:?}");
    println!("None plus one: {none:?}");

    // Matching coins and their associated data
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

    // Parsing and comparing numbers
    let guess = "42";
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let secret_number: u32 = 50;

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    // Updating a mutable struct
    let mut user1 = User {
        active: true,
        username: String::from("trainee01"),
        email: String::from("trainee@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("updated@example.com");

    println!(
        "User: {}, email: {}, active: {}, sign-ins: {}",
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count
    );
}
