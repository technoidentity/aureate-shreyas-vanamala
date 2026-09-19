/*enum Colour {
    Red,
    Blue,
    Violet,
} */

/*fn describe(colour: Colour) {
    match colour {
        Colour::Red => println!("The colour is red"),
        Colour::Blue => println!("The colour is blue"),
        Colour::Violet => println!("The colour is violet"),
    }
} */

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

//impl in Rust means implementation. It is used to add methods and associated functions to a type, including enums
impl Message {
    fn process_message(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(content) => println!("Writing: {}", content),
            Message::ChangeColour(r, g, b) => println!("Changing colour to RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("Hello, world!".into()),
        Message::ChangeColour(255, 0, 0),
    ];

    for msg in messages {
        msg.process_message();
    }
}