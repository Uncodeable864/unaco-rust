/** Original Challange: https://www.usaco.org/index.php?page=viewproblem2&cpid=855
 * A Simulation Challange <https://usaco.guide/bronze/simulation>
 * This (likley wrong) solution was written Uncodeable864 during December 2023
 * This code was verified using the samples provided by Brain Dean, the creator of the problem
 */

fn main() {
    /*
     * In reality, we shold be getting and pushing to and from the input an output files
     * but that is irrelavant b/c this solution will never actually get run
     */
    let mut bucket_list: Vec<Bucket> = vec![
        create_bucket(3, 10),
        create_bucket(4, 11),
        create_bucket(5, 12),
    ];
    let mut selector = 0;

    print_buckets(&bucket_list);

    for _i in 0..100 {
        let secondary_selector = cyclic_function(selector);
        (bucket_list[selector], bucket_list[secondary_selector]) =
            swap_bucket(&bucket_list, selector, secondary_selector);
        println!(
            "buckets for no. {}, pour {} into {}",
            _i, selector, secondary_selector
        );
        print_buckets(&bucket_list);
        selector = cyclic_function(selector);
    }
    println!("final");
    print_buckets(&bucket_list);
}

fn swap_bucket(list: &[Bucket], from: usize, to: usize) -> (Bucket, Bucket) {
    let to_bucket = &list[to];
    let from_bucket = &list[from];

    let surplus = (to_bucket.milk + from_bucket.milk) - to_bucket.capacity;

    let new_from_bucket = Bucket {
        capacity: from_bucket.capacity,
        milk: if surplus < 0 { 0 } else { surplus },
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

fn create_bucket(milk: i32, capacity: i32) -> Bucket {
    Bucket {
        milk: milk,
        capacity: capacity,
    }
}

fn cyclic_function(alpha: usize) -> usize {
    if alpha == 2 {
        return 0;
    };
    return alpha + 1;
}

fn print_buckets(buckets: &Vec<Bucket>) {
    for (index, bucket) in buckets.iter().enumerate() {
        println!(
            "Bucket {}: Milk = {}, Capacity = {}",
            index, bucket.milk, bucket.capacity
        );
    }
}
