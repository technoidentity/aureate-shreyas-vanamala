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

fn main() {
    let square = Rectangle::square(20);
    let smaller = Rectangle {
        width: 10,
        height: 15,
    };

    println!("Square area: {}", square.area());
    println!("Can hold smaller rectangle: {}", square.can_hold(&smaller));
}
