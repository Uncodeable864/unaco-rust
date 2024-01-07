/*
 * Original Challange: https://www.usaco.org/index.php?page=viewproblem2&cpid=1061;
 * from the Simulation Section at https://usaco.guide/bronze/simulation; Very Hard Difficulty
 *
 * Created by Uncodeable864, Jan 6 2024
 * to run: cargo run --bin stuck-in-a-rut
 */

fn main() {
    let mut cows: Vec<Cow> = (0..50).map(|_| Cow::create_blank_cow()).collect();
    // Above line thanks to GH Copiolot Chat

    setup_cows(&mut cows);
}

struct Cow {
    direction: CowDirection,
    initial_x: i32,
    inital_y: i32,
    current_x: i32,
    current_y: i32,
    grass: i16,
}

impl Cow {
    fn create_blank_cow() -> Cow {
        Cow {
            direction: CowDirection::NO,
            inital_y: 0,
            initial_x: 0,
            current_x: 0,
            current_y: 0,
            grass: 0,
        }
    }

    fn create_cow(direction: CowDirection, initial_x: i32, inital_y: i32) -> Cow {
        Cow {
            direction,
            initial_x,
            inital_y,
            current_x: initial_x,
            current_y: inital_y,
            grass: -1,
        }
    }
}

// One move north = (x, y + 1); east = (x + 1, y)
enum CowDirection {
    NORTH,
    EAST,
    NO,
}

fn setup_cows(cows: &mut Vec<Cow>) {
    cows[0] = Cow::create_cow(CowDirection::EAST, 3, 5);
    cows[1] = Cow::create_cow(CowDirection::NORTH, 5, 3);
    cows[02] = Cow::create_cow(CowDirection::EAST, 4, 6);
    cows[03] = Cow::create_cow(CowDirection::EAST, 10, 4);
    cows[04] = Cow::create_cow(CowDirection::NORTH, 11, 2);
    cows[05] = Cow::create_cow(CowDirection::NORTH, 8, 1);
}

fn check_for_infinite_cows(cows: Vec<Cow>) -> Vec<Cow> {
    // DONE: Check all pairs of cows

    // DONE: Check to see if cow.grass == -1 (intersection is undetermined)

    // Go through all cows to see if they will ever intersect

    let cow_copy = &cows;

    for (i, cow) in &mut cows.iter().enumerate() {
        if cow.grass >= 0 {
        } else {
            for (j, cow) in cow_copy.iter().enumerate() {
                if i != j {}
            }
        }
    }

    return cows;
}

fn will_cows_intersect() -> (bool, i32, i32) {
    return (false, 0, 0);
}
