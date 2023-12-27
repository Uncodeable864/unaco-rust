/** https://www.usaco.org/index.php?page=viewproblem2&cpid=855 */

fn main() {
    let main_bucket = create_bucket(5, 10);
    let into_bucket = create_bucket(3, 12);

    let bucket_list = [main_bucket, into_bucket];

    print_buckets(swap_bucket(&bucket_list, 0, 1));
}

fn swap_bucket(list: &[Bucket], from: usize, to: usize) -> (Bucket, Bucket) {
    let to_bucket = &list[to];
    let from_bucket = &list[from];

    let new_from_bucket: Bucket = Bucket {
        capacity: from_bucket.capacity,
        milk: (to_bucket.milk - from_bucket.capacity).clamp(0, 100),
    };

    let new_to_bucket: Bucket = Bucket {
        capacity: to_bucket.capacity,
        milk: (from_bucket.milk + to_bucket.milk).clamp(0, to_bucket.capacity),
    };

    return (new_from_bucket, new_to_bucket)
}

struct Bucket {
    capacity: i8,
    milk: i8,
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
        milk: 0
    }
}

fn create_bucket(milk: i8, capacity: i8) -> Bucket {
    Bucket {
        milk: milk,
        capacity: capacity
    }
}