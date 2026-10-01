use std::io;

fn leap_year(year: i32) -> bool {
    if year % 4 != 0 {
        return false;
    }
    if year % 100 != 0 {
        return true;
    }
    if year % 400 != 0 {
        return false;
    }
    true
}

fn main() {
    let mut input = String::new();

    println!("Enter a year:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let year: i32 = input.trim().parse().expect("Please enter a valid year");

    if leap_year(year) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year", year);
    }
}
