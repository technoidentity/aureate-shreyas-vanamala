use std::cmp::Ordering;

fn binary_search<T: Ord>(items: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = items.len();

    while low < high {
        let mid = low + (high - low) / 2;

        match items[mid].cmp(target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
        }
    }

    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9];
    let target = 7;

    assert_eq!(binary_search(&arr, &target), Some(3));
    assert_eq!(binary_search(&arr, &4), None);

    println!("Binary search checks passed.");
}
