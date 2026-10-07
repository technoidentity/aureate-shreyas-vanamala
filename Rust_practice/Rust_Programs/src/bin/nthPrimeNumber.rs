use std::io;

fn main() {
    println!("Enter the prime position (1 or greater):");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u32 = match input.trim().parse() {
        Ok(number) if number > 0 => number,
        _ => {
            println!("Please enter a positive whole number.");
            return;
        }
    };

    let result = nth_prime(n);
    println!("Prime number at position {n} is {result}");
}

fn nth_prime(n: u32) -> u32 {
    let mut count = 0;
    let mut num = 2;

    while count < n {
        if is_prime(num) {
            count += 1;
        }
        num += 1;
    }

    num - 1
}

fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}
