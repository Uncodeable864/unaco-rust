/*
 * Original Challange: https://www.usaco.org/index.php?page=viewproblem2&cpid=664; from the Simulation
 * Section at https://usaco.guide/bronze/simulation; Normal Difficulty
 *
 * Created by Uncodeable864, Jan 1 2024
 * to run: cargo run --bin block-game
 */
fn main() {
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

    let combinations = calculate_permutations(&board_list, board_permutations as usize);

    for z in combinations {
        println!("{}", z);
    }

    // go through all letters; find max per letter in all; save and return;
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
