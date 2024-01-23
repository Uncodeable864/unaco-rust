/*
 * Original Challange: <link>;
 * from the <section> Section at <link2>; <diff> Difficulty
 *
 * Created by Uncodeable864, <date>
 * to run: cargo run --bin <run-path>
 *
 * Example run time: <time>µs
 *
 * O: <big-O>
 */

fn main() {
    use std::time::Instant; // Benchmarking solution curtosy of @ideasman42 on SO
    let now = Instant::now();

    /*
    As with all solutions here, I am using stdout to print the result, and additionally using variables to
    initilaize the paramaters based off of `blocks.in.` The code is written is such a way so that it would
    be trivail to implement a proper I/O system
    */

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
