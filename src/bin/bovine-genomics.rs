/*
 * Original Challange: https://www.usaco.org/index.php?page=viewproblem2&cpid=736;
 * from the Basic Complete Search Section Section at https://usaco.guide/bronze/intro-complete; Normal Difficulty
 *
 * Created by Uncodeable864, Jan. 22, 2024
 * to run: cargo run --bin bronze/basic_complete_search/bovine-genomics.rs
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
