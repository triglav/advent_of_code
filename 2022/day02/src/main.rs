use std::io;

// opponent: A for Rock, B for Paper, and C for Scissors
// myself:   X for Rock, Y for Paper, and Z for Scissors

// 1 for Rock, 2 for Paper, and 3 for Scissors
fn points_for_shape(myself: u8) -> u32 {
    match myself {
        0 => 1,
        1 => 2,
        2 => 3,
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

fn points_for_outcome(a: u8, b: u8) -> u32 {
    if a == b {
        3
    } else if (a + 1) % 3 == b {
        6
    } else {
        0
    }
}

// 0 if you lost, 3 if the round was a draw, and 6 if you won
fn points_for_round(opponent: char, myself: char) -> u32 {
    let a = shape_to_number(opponent);
    let b = shape_to_number(myself);
    points_for_outcome(a, b) + points_for_shape(b)
}

// X means you need to lose
// Y means you need to end the round in a draw
// Z means you need to win

fn points_for_round2(opponent: char, result: char) -> u32 {
    let a = shape_to_number(opponent);
    let b = match result {
        'X' => {
            if a == 0 {
                2
            } else {
                a - 1
            }
        }
        'Y' => a,
        'Z' => (a + 1) % 3,
        _ => panic!("invalid result"),
    };
    points_for_outcome(a, b) + points_for_shape(b)
}

fn main() {
    let (r1, r2) = io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|s| s.chars().next().unwrap())
                .collect()
        })
        .map(|t: Vec<char>| (points_for_round(t[0], t[1]), points_for_round2(t[0], t[1])))
        .fold((0, 0), |a, c| (a.0 + c.0, a.1 + c.1));
    println!("{}", r1);
    println!("{}", r2);
}
