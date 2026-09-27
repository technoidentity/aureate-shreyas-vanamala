pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut total = 0;

    for number in 1..limit {
        for &factor in factors {
            if factor != 0 && number % factor == 0 {
                total += number;
                break;
            }
        }
    }

    total
}

fn main() {
    let points = sum_of_multiples(20, &[3, 5]);
    println!("Energy points: {points}");
}
