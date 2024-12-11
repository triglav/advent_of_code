use std::{collections::HashMap, io};

#[derive(Debug)]
struct Rock {
    value: u64,
    count: u64,
}

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

fn compact(rocks: Vec<Rock>) -> Vec<Rock> {
    rocks
        .into_iter()
        .fold(HashMap::new(), |mut acc, rock| {
            acc.entry(rock.value)
                .and_modify(|c| *c += rock.count)
                .or_insert(rock.count);
            acc
        })
        .into_iter()
        .map(|(value, count)| Rock { value, count })
        .collect()
}

fn blink(rocks: Vec<Rock>) -> Vec<Rock> {
    compact(rocks)
        .into_iter()
        .flat_map(|r| {
            blink_rock(r.value).into_iter().map(move |rr| Rock {
                value: rr,
                count: r.count,
            })
        })
        .collect()
}

fn main() {
    let input = io::stdin().lines().next().unwrap().unwrap();
    let rocks = input
        .split_whitespace()
        .map(|s| Rock {
            value: s.parse::<u64>().unwrap(),
            count: 1,
        })
        .collect::<Vec<_>>();

    let mut rocks = rocks;
    for _ in 0..25 {
        rocks = blink(rocks);
    }
    let r1 = rocks.iter().map(|r| r.count).sum::<u64>();
    println!("{}", r1);
    for _ in 0..50 {
        rocks = blink(rocks);
    }
    let r2 = rocks.iter().map(|r| r.count).sum::<u64>();
    println!("{}", r2);
}
