/*
fn main() {
    let sum = add(5, 10);
    let difference = subtract(10, 5);
    let product = multiply(4, 10);
    let division = quotient(10, 2);

    println!("The sum is: {sum}");
    println!("The difference is: {difference}");
    println!("The product is: {product}");
    println!("The quotient is: {division}");
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
*/

fn main() {
    variable();
}

fn variable(){
    /*println!("The value of x is: {x}");
    let x =5; */

    /* let f =true;

    let t = false;
    println!("f is {f} and t is {t}"); */
    /*let c = 'b';
    let z = 5.0;
    println!("c is {c} and z is {z}"); */

    /*let tuple = (300, 5.3, 'c');
    let (_x, _y, z) = tuple;
    println!("The value of z is: {z}"); */

    /*let a = ['2','4','8','7','5'];
    let first = a[0];
    println!("The value of first is: {first}");
    println!("The value of a is: {a:?}"); */

    /*let condition = true;
    let number = if condition { 4 } else { 6 };
    println!("The value of number is: {number}"); */

/* let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }; */


    /*let a = [5; 10];  //length of array is 5, 10 is the value of each element in the array

    let mut sum = 0;

    for x in a {

        sum += x;

    }

    println!("{sum}"); */

    /*let scale = 2;
    let width = dbg!(30 * scale); */

/*#[derive(Debug)]
struct Rectangle {
width: u32,
height: u32,
}
fn main() {
    let rect1 = Rectangle {

    width: 30,

    height: 50,
};
let a = area(rect1);{

  println!("{} * {} = {}", rect1.width, rect1.height, a);

}


fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */
    /*let a = [9, 2, 9, 4, 5];
    let mut index = 0;

    for _element in a {
        println!("the value is: {}", a[index]);

        index += 1;


let fixed_text = "hello";
let mut owned_text = String::from("hello");
owned_text.push_str(", Rust");
println!("{owned_text}");

{
    let message = String::from("hello");
    println!("{message}");
    let x = 5;
    let y = x;
println!("x = {x}, y = {y}");
}

    } */
    
    struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1.email);



    println!(
        "User created: username={} email={} active={} sign_in_count={}"
        , &user1.username, &user1.email, user1.active, user1.sign_in_count
    );
        /*fn take_ownership(text: String) {
    println!("{text}");
}
fn make_copy(number: i32) {
    println!("{number}");
} */
fn main() {
    let message = String::from("hello"); // message moves into the function and is no longer valid here
    let number = 10;
    println!("{number}"); // number is still valid
    }
    fn create_message() -> String {
    String::from("hello")
}
fn return_message(text: String) -> String {
    text
}

    let first = create_message();
    let second = return_message(first);
    println!("{second}");
}

}
