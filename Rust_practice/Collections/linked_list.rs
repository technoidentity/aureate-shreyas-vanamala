struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn main() {
    let first = 10;
    let third = Node {
        value: 30,
        next: None,
    };
    let second = Node {
        value: 20,
        next: Some(Box::new(third)),
    };
    let node = Node {
        value: first,
        next: Some(Box::new(second)),
    };

    let mut current = Some(&node);

    while let Some(n) = current {
        println!("{}", n.value);
        current = n.next.as_deref();
    }
}
