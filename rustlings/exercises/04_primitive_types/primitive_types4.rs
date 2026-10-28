fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    println!("Nice slice: {:?}", nice_slice);
}

fn main() {
    slice_out_of_array();
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array_test() {
        let a = [1, 2, 3, 4, 5];

        let nice_slice = &a[1..4];

        assert_eq!([2, 3, 4], nice_slice);
    }
}
