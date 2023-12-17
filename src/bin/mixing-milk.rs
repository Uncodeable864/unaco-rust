/** https://www.usaco.org/index.php?page=viewproblem2&cpid=855 */

fn main() {
    let mut vec = vec![
        Bucket {
            capacity: 10,
            milk: 3,
        },
        Bucket {
            capacity: 11,
            milk: 4,
        },
        Bucket {
            capacity: 12,
            milk: 5,
        },
    ];
}

fn swap_bucket(list: &mut [Bucket], from: usize, to: usize) -> Bucket {
    let from_bucket = &mut list[from];
    let to_bucket = &list[to];

    let mut new_bucket: Bucket = Bucket {
        capacity: 0,
        milk: 0,
    };

    if !(from_bucket.capacity == from_bucket.milk) && !(to_bucket.capacity == to_bucket.milk) {
        // Calculate remaing capacity of bucket
        new_bucket.milk += from_bucket.milk + to_bucket.milk;
        new_bucket.capacity = from_bucket.milk;

        from_bucket.capacity = 0;

        if new_bucket.milk > new_bucket.capacity {
            let change_amount = new_bucket.milk - new_bucket.capacity;
            from_bucket.capacity = change_amount;
        }
        return new_bucket;
    }
    return Bucket {
        capacity: 0,
        milk: (0),
    };
}

struct Bucket {
    capacity: usize,
    milk: usize,
}
