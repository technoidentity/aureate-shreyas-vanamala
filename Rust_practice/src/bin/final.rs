enum Colour {
    Red,
    Blue,
    Violet,
}

fn describe(colour: Colour) {
    match colour {
        Colour::Red => println!("The colour is red"),
        Colour::Blue => println!("The colour is blue"),
        Colour::Violet => println!("The colour is violet"),
    }
}

fn main() {
    describe(Colour::Blue);
}