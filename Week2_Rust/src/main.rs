impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

let square = Rectangle::square(20);

enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn route(address: IpAddr) {
    match address {
        IpAddr::V4(value) => println!("IPv4: {value}"),
        IpAddr::V6(value) => println!("IPv6: {value}"),
    }
}

impl Message {
    fn call(&self) {
        println!("A message was received");
    }
}
let message = Message::Write(String::from("hello"));
message.call();

fn describe(value: Option<i32>) {
    let Some(number) = value else {
        println!("No number was provided");
        return;
    };

    println!("The number is {number}");
}

let config_max = Some(3_u8);

if let Some(max) = config_max {
    println!("The maximum is {max}");
} else {
    println!("No maximum value was set");
}

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

fn plus_one(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(number) => Some(number + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Quarter(UsState),
}

match coin {
    Coin::Penny => 1,
    Coin::Quarter(state) => {
        println!("State quarter from {state:?}");
        25
    }
}

use std::cmp::Ordering;

let guess: u32 = guess.trim().parse().expect("Please type a number!");

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}


use rand::Rng;

let secret_number = rand::rng().random_range(1..=100);


let mut user1 = User {
    active: true,
    username: String::from("trainee01"),
    email: String::from("trainee@example.com"),
    sign_in_count: 1,
};

user1.email = String::from("updated@example.com");


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
