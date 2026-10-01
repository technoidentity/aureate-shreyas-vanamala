#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn quotient(x: i32, y: i32) -> i32 {
    x / y
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn take_ownership(text: String) {
    println!("Owned message: {text}");
}

fn make_copy(number: i32) {
    println!("Copied number: {number}");
}

fn create_message() -> String {
    String::from("hello")
}

fn return_message(text: String) -> String {
    text
}

fn main() {
    // Arithmetic functions
    let sum = add(5, 10);
    let difference = subtract(10, 5);
    let product = multiply(4, 10);
    let division = quotient(10, 2);

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {division}");

    variable();
}

fn variable() {
    // Define a variable before using it
    let x = 5;
    println!("The value of x is: {x}");

    // Booleans
    let f = true;
    let t = false;
    println!("f is {f} and t is {t}");

    // Character and floating-point values
    let c = 'b';
    let z = 5.0;
    println!("c is {c} and z is {z}");

    // Tuple destructuring
    let tuple = (300, 5.3, 'c');
    let (_x, _y, z) = tuple;
    println!("The value of z is: {z}");

    // Arrays and indexing
    let a = ['2', '4', '8', '7', '5'];
    let first = a[0];
    println!("The first element is: {first}");
    println!("The array is: {a:?}");

    // if as an expression
    let condition = true;
    let number = if condition { 4 } else { 6 };
    println!("The value of number is: {number}");

    // Returning a value from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The loop result is: {result}");

    // Ten elements, each containing 5
    let a = [5; 10];
    let mut sum = 0;

    for x in a {
        sum += x;
    }

    println!("The array sum is: {sum}");

    // dbg! prints a value and returns it
    let scale = 2;
    let width = dbg!(30 * scale);
    println!("The scaled width is: {width}");

    // Borrow the rectangle so it remains usable afterward
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle_area = area(&rect1);

    println!(
        "{} * {} = {}",
        rect1.width, rect1.height, rectangle_area
    );
    println!("Rectangle: {rect1:?}");

    // Iterate directly over array elements
    let a = [9, 2, 9, 4, 5];

    for element in a {
        println!("The value is: {element}");
    }

    // String literal and owned String
    let fixed_text = "hello";
    let mut owned_text = String::from("hello");
    owned_text.push_str(", Rust");

    println!("Fixed text: {fixed_text}");
    println!("Owned text: {owned_text}");

    // Scope and copying integers
    {
        let message = String::from("hello");
        println!("{message}");

        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");
    }

    // Creating and reading a struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    println!(
        "User created: username={} email={} active={} sign_in_count={}",
        user1.username,
        user1.email,
        user1.active,
        user1.sign_in_count
    );

    // String moves; integer copies
    let message = String::from("hello");
    take_ownership(message);
    // message cannot be used here because ownership moved.

    let number = 10;
    make_copy(number);
    println!("The original number is still usable: {number}");

    // Returning ownership from functions
    let first = create_message();
    let second = return_message(first);
    // first has moved into return_message.

    println!("Returned message: {second}");
}
