fn square_grains(square: u32) -> u64 {
    2u64.pow(square - 1)
}

fn total_grains() -> u64 {
    let mut total = 0;
    for square in 1..=64 {
        total += square_grains(square);
    }
    total
}

fn main() {
    let _total = total_grains();
    println!("Total grains on the chessboard: {}", _total);
}
