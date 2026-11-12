#[derive(Debug)]
enum Message {
    // Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
