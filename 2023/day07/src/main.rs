use std::{collections::HashMap, io};

use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    pub hand_type: HandType,
    pub hand_type2: HandType,
    pub strength: u32,
    pub strength2: u32,
    pub bid: u32,
}

fn get_hand_type(cards: &str) -> HandType {
    let cards = cards.chars().fold(HashMap::new(), |mut a, c| {
        a.insert(c, a.get(&c).unwrap_or(&0) + 1u8);
        a
    });
    let mut card_counts = cards.values().collect::<Vec<_>>();
    card_counts.sort();
    match card_counts[..] {
        [5] => HandType::FiveOfAKind,
        [1, 4] => HandType::FourOfAKind,
        [2, 3] => HandType::FullHouse,
        [1, 1, 3] => HandType::ThreeOfAKind,
        [1, 2, 2] => HandType::TwoPair,
        [1, 1, 1, 2] => HandType::OnePair,
        [1, 1, 1, 1, 1] => HandType::HighCard,
        _ => panic!("Invalid hand"),
    }
}

fn get_hand_type2(cards: &str) -> HandType {
    let s0: String = cards.chars().filter(|c| *c != 'J').collect();
    let l = cards.len() - s0.len();
    if l == 0 {
        return get_hand_type(cards);
    }
    let cards = "AKQT98765432";
    (0..l)
        .map(|_| cards.chars())
        .multi_cartesian_product()
        .map(|v| {
            let s = v.into_iter().collect::<String>();
            let s = format!("{s0}{s}");
            get_hand_type(&s)
        })
        .min()
        .unwrap()
}

fn parse(
    line: &str,
    card_strength: &HashMap<char, u32>,
    card_strength2: &HashMap<char, u32>,
) -> Hand {
    let mut t = line.split(' ');
    let s = t.next().unwrap();
    let bid = t.next().unwrap().parse::<u32>().unwrap();
    Hand {
        hand_type: get_hand_type(s),
        hand_type2: get_hand_type2(s),
        strength: s
            .chars()
            .fold(0u32, |a, c| a * 13 + card_strength.get(&c).unwrap()),
        strength2: s
            .chars()
            .fold(0u32, |a, c| a * 13 + card_strength2.get(&c).unwrap()),
        bid,
    }
}

fn main() {
    let card_strength = HashMap::from([
        ('A', 12u32),
        ('K', 11u32),
        ('Q', 10u32),
        ('J', 9u32),
        ('T', 8u32),
        ('9', 7u32),
        ('8', 6u32),
        ('7', 5u32),
        ('6', 4u32),
        ('5', 3u32),
        ('4', 2u32),
        ('3', 1u32),
        ('2', 0u32),
    ]);
    let card_strength2 = HashMap::from([
        ('A', 12u32),
        ('K', 11u32),
        ('Q', 10u32),
        ('T', 9u32),
        ('9', 8u32),
        ('8', 7u32),
        ('7', 6u32),
        ('6', 5u32),
        ('5', 4u32),
        ('4', 3u32),
        ('3', 2u32),
        ('2', 1u32),
        ('J', 0u32),
    ]);

    let mut hands = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str(), &card_strength, &card_strength2))
        .collect::<Vec<_>>();

    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then_with(|| b.strength.cmp(&a.strength))
    });

    let r1 = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum::<u32>();
    println!("{}", r1);

    hands.sort_by(|a, b| {
        a.hand_type2
            .cmp(&b.hand_type2)
            .then_with(|| b.strength2.cmp(&a.strength2))
    });

    let r2 = hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum::<u32>();
    println!("{}", r2);
}
