/*
 * Original Challange: https://www.usaco.org/index.php?page=viewproblem2&cpid=664; from the Simulation
 * Section at https://usaco.guide/bronze/simulation; Normal Difficulty
 *
 * Created by Uncodeable864, Jan 1 2024
 * to run: cargo run --bin block-game
 *
 * Code run time: ~1.7ms
 */

use regex::Regex;
use std::char; // The USACO languages all have regex, so I amm using a crate for it hear

fn main() {
    use std::time::Instant; // Benchmarking solution curtosy of @ideasman42 on SO
    let now = Instant::now();

    /*
    As with all solutions here, I am using stdout to print the result, and additionally using variables to
    initilaize the paramaters based off of `blocks.in.` The code is written is such a way so that it would
    be trivail to implement a proper I/O system
    */

    let board_count = 3;
    let board_permutations = i32::pow(2, board_count);

    let board_list: Vec<Board> = vec![
        Board::create_board("fox", "box"),
        Board::create_board("dog", "cat"),
        Board::create_board("car", "bus"),
    ];

    let combinations: Vec<String> =
        calculate_permutations(&board_list, board_permutations as usize);

    for z in &combinations {
        println!("{}", z);
    }

    let (regex_array, alphabet) = create_alphabet_regex();

    let mut count_array: [usize; 26] = [0; 26];

    // go through all letters; find max per letter in all; save and return;
    for (_, permutation) in combinations.iter().enumerate() {
        for (letter_id, _) in alphabet.iter().enumerate() {
            let appearance_count = regex_array
                .get(letter_id)
                .unwrap()
                .find_iter(&permutation)
                .count();
            if count_array[letter_id] < appearance_count {
                count_array[letter_id] = appearance_count;
            }
        }
    }

    print_array(&count_array);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn create_alphabet_regex() -> (Vec<Regex>, Vec<char>) {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    (
        alphabet
            .iter()
            .map(|&c| Regex::new(&format!("{}", c)).unwrap())
            .collect(),
        alphabet,
    )
}

fn calculate_permutations(list: &[Board], board_permutations: usize) -> Vec<String> {
    let mut permutations: Vec<String> = Vec::new();

    for i in 0..board_permutations {
        let mut combination = "".to_string();

        // convert i to binary
        for (j, n) in (0..list.len())
            .rev()
            .map(|n| (i as i8 >> n) & 1)
            .into_iter()
            .enumerate()
        // Iterator courtesy for @Jmb on SO (https://stackoverflow.com/a/74163801)
        {
            if n == 1 {
                combination = [combination, list[j].front.to_string()].join(" ");
            } else if n == 0 {
                combination = [combination, list[j].back.to_string()].join(" ");
            } else {
                println!("error");
            }
        }

        permutations.push(combination);
    }

    permutations
}

struct Board {
    front: String,
    back: String,
}

impl Board {
    fn create_board(f: &str, b: &str) -> Board {
        Board {
            front: f.to_string(),
            back: b.to_string(),
        }
    }
}

fn print_array(arr: &[usize]) {
    for (index, num) in arr.iter().enumerate() {
        println!("array[{}] = {}", index, num);
    }
}
