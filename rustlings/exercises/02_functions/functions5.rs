// Fix the function body without changing the signature.
fn square(num: i32) -> i64 {
    i64::from(num) * i64::from(num)
}

fn main() {
    let answer = square(7);
    println!("The square of 7 is {answer}");
}
