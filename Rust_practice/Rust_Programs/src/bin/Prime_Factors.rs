use std::io;

fn prime_factors(n: i32) -> Vec<i32> {
    let mut factors = vec![];
    let mut n = n;
    let mut i = 2;
    while i <= n {
        if n % i == 0 {
            factors.push(i);
            n /= i;
        } else {
            i += 1;
        }
    }
    factors
}

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: i32 = input.trim().parse().unwrap();
    let factors = prime_factors(n);
    println!("{:?}", factors);
}
