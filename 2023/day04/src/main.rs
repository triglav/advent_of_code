use std::{collections::HashMap, io};

struct Card {
    pub id: u32,
    pub numbers: Vec<u32>,
    pub winning: Vec<u32>,
}

fn parse(line: &str) -> Card {
    let t = line.split(&[':', '|']).collect::<Vec<_>>();
    let id = t[0].split(' ').last().unwrap().parse::<u32>().unwrap();
    let mut numbers = t[1]
        .trim()
        .split(' ')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();
    numbers.sort();
    let mut winning = t[2]
        .trim()
        .split(' ')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<_>>();
    winning.sort();
    Card {
        id,
        numbers,
        winning,
    }
}

fn count_matching(numbers: &[u32], winning: &[u32]) -> u32 {
    let count = numbers
        .iter()
        .filter_map(|n| winning.binary_search(n).ok())
        .count() as u32;
    count
}

fn count_winning(numbers: &[u32], winning: &[u32]) -> u32 {
    let count = count_matching(numbers, winning);
    match count {
        0 => 0u32,
        1 => 1u32,
        _ => 2u32.pow(count - 1),
    }
}

fn main() {
    let cards = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();
    let r1 = cards
        .iter()
        .map(|c| count_winning(&c.numbers, &c.winning))
        .sum::<u32>();
    println!("{}", r1);

    let ids = cards.iter().map(|c| c.id).collect::<Vec<_>>();
    let mut m = HashMap::<u32, u32>::new();
    ids.iter().for_each(|id| {
        m.insert(*id, 1);
    });
    let mut grcka = 0u32;
    cards.iter().for_each(|c| {
        let w0 = *m.get(&c.id).unwrap();
        let w = count_matching(&c.numbers, &c.winning);
        (c.id + 1..=c.id + w).for_each(|i| {
            *m.get_mut(&i).unwrap() += w0;
        });
        if w == 0 && grcka == 0 {
            grcka = c.id;
        }
    });
    let r2 = m.values().sum::<u32>();
    println!("{}", r2);
}
