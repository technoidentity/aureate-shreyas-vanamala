fn merge_sort(items: &mut [i32]) {
    let len = items.len();
    if len <= 1 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut items[..mid]);
    merge_sort(&mut items[mid..]);

    let mut merged = Vec::with_capacity(len);
    let (mut left, mut right) = (0, mid);

    while left < mid && right < len {
        if items[left] < items[right] {
            merged.push(items[left]);
            left += 1;
        } else {
            merged.push(items[right]);
            right += 1;
        }
    }
    merged.extend_from_slice(&items[left..mid]);
    merged.extend_from_slice(&items[right..len]);
    items.copy_from_slice(&merged);
}

fn main() {
    let mut items = vec![38, 27, 43, 3, 9, 82, 10];
    println!("Unsorted: {:?}", items);
    merge_sort(&mut items);
    println!("Sorted: {:?}", items);
}
