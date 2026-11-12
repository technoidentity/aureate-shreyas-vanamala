#[derive(Debug)]
struct HashTable {
    buckets: Vec<Vec<usize>>,
}

impl HashTable {
    fn new(bucket_count: usize) -> Self {
        assert!(bucket_count > 0, "bucket count must be greater than zero");

        Self {
            buckets: vec![Vec::new(); bucket_count],
        }
    }

    fn bucket_index(&self, key: usize) -> usize {
        key % self.buckets.len()
    }

    fn insert(&mut self, key: usize) {
        let index = self.bucket_index(key);

        // Avoid storing the same key twice.
        if !self.buckets[index].contains(&key) {
            self.buckets[index].push(key);
        }
    }

    fn contains(&self, key: usize) -> bool {
        let index = self.bucket_index(key);
        self.buckets[index].contains(&key)
    }
}

fn main() {
    let mut table = HashTable::new(5);

    for key in [12, 17, 4] {
        table.insert(key);
    }

    println!("Bucket 2: {:?}", table.buckets[2]);
    println!("Bucket 4: {:?}", table.buckets[4]);
    println!("Contains 17: {}", table.contains(17));
    println!("Contains 22: {}", table.contains(22));
}
