use std::cmp::Ordering;

fn main() {
    let dice_roll = 9;

    match dice_roll {
        3 => println!("Move three spaces"),
        7 => println!("Collect a reward"),
        other => println!("Move {other} spaces"),
    }

    match dice_roll {
        3 => println!("Special action"),
        _ => (),
    }

    let guess = "42";
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    let secret_number: u32 = 50;

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
