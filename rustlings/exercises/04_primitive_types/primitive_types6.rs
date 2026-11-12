fn main() {
    // You can optionally experiment here.
    indexing_tuple();
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);

    let second = numbers.1;

    println!("Second number: {second}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
