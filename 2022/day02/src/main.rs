use std::io;

// opponent: A for Rock, B for Paper, and C for Scissors
// myself:   X for Rock, Y for Paper, and Z for Scissors

// 1 for Rock, 2 for Paper, and 3 for Scissors
fn points_for_shape(myself: char) -> u8 {
    match myself {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("invalid shape"),
    }
}

fn shape_to_number(shape: char) -> u8 {
    match shape {
        'A' => 0, // Rock
        'B' => 1, // Paper
        'C' => 2, // Scissors
        'X' => 0, // Rock
        'Y' => 1, // Paper
        'Z' => 2, // Scissors
        _ => panic!("invalid shape"),
    }
}

// 0 if you lost, 3 if the round was a draw, and 6 if you won
fn points_for_round(opponent: char, myself: char) -> u32 {
    let a = shape_to_number(opponent);
    let b = shape_to_number(myself);
    let points_for_outcome = if a == b {
        3
    } else if (a + 1) % 3 == b {
        6
    } else {
        0
    };
    (points_for_outcome + points_for_shape(myself)).into()
}

fn main() {
    let r = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|s| s.chars().next().unwrap())
                .collect()
        })
        .map(|t: Vec<char>| points_for_round(t[0], t[1]))
        .sum::<u32>();
    println!("{}", r);
}
