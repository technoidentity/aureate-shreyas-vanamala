use std::cmp::Ordering;
use std::fmt::Display;

struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T: Ord> Node<T> {
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = self.right.as_mut() {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
            Ordering::Equal => {} // Ignore duplicates
        }
    }
}

fn inorder<T: Display>(root: &Option<Box<Node<T>>>) {
    if let Some(node) = root {
        inorder(&node.left);
        print!("{} ", node.value);
        inorder(&node.right);
    }
}

fn main() {
    let mut root = Node::new(8);

    for value in [5, 2, 8, 1, 3] {
        root.insert(value);
    }

    let root = Some(Box::new(root));
    inorder(&root);
    println!();
}

struct HashTable {
    buckets: Vec<Vec<usize>>,
}

impl HashTable {
    fn new(bucket_count: usize) -> Self {
        assert!(bucket_count > 0);
        Self {
            buckets: vec![Vec::new(); bucket_count],
        }
    }

    fn insert(&mut self, key: usize) {
        let index = key % self.buckets.len();
        self.buckets[index].push(key);
    }
}
