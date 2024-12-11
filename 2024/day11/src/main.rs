use std::io;

fn blink_rock(rock: u64) -> Vec<u64> {
    if rock == 0 {
        return vec![1];
    }
    let s = rock.to_string();
    if s.len() % 2 == 0 {
        let r1 = s[0..s.len() / 2].parse::<u64>().unwrap();
        let r2 = s[s.len() / 2..].parse::<u64>().unwrap();
        return vec![r1, r2];
    }
    vec![rock * 2024]
}

fn blink(rocks: Vec<u64>) -> Vec<u64> {
    rocks.into_iter().flat_map(blink_rock).collect()
}

fn main() {
    let input = io::stdin().lines().next().unwrap().unwrap();
    let rocks = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut rocks = rocks;
    for _ in 0..25 {
        rocks = blink(rocks);
    }
    let r1 = rocks.len();
    println!("{}", r1);
}
