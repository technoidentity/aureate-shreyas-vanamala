fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let n = 0..8;
    for i in n {
        let result = square(i);
        println!("The square of {} is {}", i, result);
        let diff = result - square(i - 1);
        println!(
            "The difference between the square of {} and the square of {} is {}",
            i,
            i - 1,
            diff
        );
        let sum = result + square(i - 1);
        println!(
            "The sum of the square of {} and the square of {} is {}",
            i,
            i - 1,
            sum
        );
    }
}
