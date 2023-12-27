/** https://www.usaco.org/index.php?page=viewproblem2&cpid=855
 * This (likley wrong) solution was written Uncodeable864 during December 2023
 */

fn main() {

    // Create a set of 3 buckets
    let mut bucket_list: Vec<Bucket> = vec![create_bucket(2, 10), create_bucket(5, 7), create_bucket(4, 15)];

}

fn swap_bucket(list: &[Bucket], from: usize, to: usize) -> (Bucket, Bucket) {
    let to_bucket = &list[to];
    let from_bucket = &list[from];

    let surplus = (to_bucket.milk + from_bucket.milk) - to_bucket.capacity;

    let new_from_bucket = Bucket {
        capacity: from_bucket.capacity,
        milk: if surplus < 0 {0} else {surplus},
    };

    let new_to_bucket: Bucket = Bucket {
        capacity: to_bucket.capacity,
        milk: (from_bucket.milk + to_bucket.milk).clamp(0, to_bucket.capacity),
    };

    return (new_from_bucket, new_to_bucket);
}

struct Bucket {
    capacity: i32,
    milk: i32,
}

fn print_buckets(buckets: (Bucket, Bucket)) {
    println!("First bucket:");
    println!("  Capacity: {}", buckets.0.capacity);
    println!("  Milk: {}", buckets.0.milk);

    println!("Second bucket:");
    println!("  Capacity: {}", buckets.1.capacity);
    println!("  Milk: {}", buckets.1.milk);
}

fn empty_bucket() -> Bucket {
    Bucket {
        capacity: 0,
        milk: 0,
    }
}

fn create_bucket(milk: i32, capacity: i32) -> Bucket {
    Bucket {
        milk: milk,
        capacity: capacity,
    }
}
