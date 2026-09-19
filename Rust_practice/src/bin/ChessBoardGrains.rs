fn grains_on_square(square: u32) -> u64 {
    2u64.pow(square - 1)
}

fn total_grains() -> u64 {
    let mut total = 0;

    for square in 1..=64 {
        total += grains_on_square(square);
    }

    total
}

fn main() {
    let square = 10;

    println!("Grains on square {}: {}", square, grains_on_square(square));

    println!("Total grains on the chessboard: {}", total_grains());
}
