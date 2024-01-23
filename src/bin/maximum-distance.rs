/*
 * Original Challange: https://codeforces.com/gym/102951/problem/A;
 * from the Basic Complete Search Section at https://usaco.guide/bronze/intro-complete; Easy Difficulty
 *
 * Created by Uncodeable864, Jan 7 2024
 * to run: cargo run --bin maximum-distance
 *
 * Example run time: ~59.1µs (average of 5), ~45.6µs (excluding outlier)
 *
 * O: O(n!)
 */

fn main() {
    use std::time::Instant; // Benchmarking solution curtosy of @ideasman42 on SO
    let now = Instant::now();

    /*
    As with all solutions here, I am using stdout to print the result, and additionally using variables to
    initilaize the paramaters based off of `blocks.in.` The code is written is such a way so that it would
    be trivail to implement a proper I/O system
    */

    let all_points: Vec<CoordiantPoint> = vec![
        CoordiantPoint::create_point(321, 404),
        CoordiantPoint::create_point(-15, 373),
        CoordiantPoint::create_point(-525, 990),
    ];

    let mut biggest_distance: i64 = 0;

    for point in &all_points {
        for other_point in &all_points {
            let distance = point.calclate_distance(other_point);

            println!(
                "Distance between ({}) and ({}) is {}",
                point.x, other_point.x, distance
            );

            if distance > biggest_distance {
                biggest_distance = distance
            }
        }
    }

    println!("Largest jump is {} units", biggest_distance);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

struct CoordiantPoint {
    x: i64,
    y: i64,
}

impl CoordiantPoint {
    fn create_point(x: i64, y: i64) -> CoordiantPoint {
        CoordiantPoint { x, y }
    }

    fn calclate_distance(&self, other_point: &CoordiantPoint) -> i64 {
        let x_distance = i64::pow(self.x - other_point.x, 2);
        let y_distance = i64::pow(self.y - other_point.y, 2);

        return i64::abs(x_distance + y_distance);
    }
}
