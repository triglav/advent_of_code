use core::panic;
use std::{cmp::Ordering, io};

const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const WORDS_TO_NUMBER: &[u32] = &[1, 2, 3, 4, 5, 6, 7, 8, 9];

#[derive(Clone, Copy, Debug)]
struct Finding {
    pub number: u32,
    pub pos: usize,
}

fn find_words(s: &str) -> (Option<Finding>, Option<Finding>) {
    let first_indices = WORDS
        .iter()
        .map(|w| s.find(w))
        .enumerate()
        .filter(|(_i, pos)| pos.is_some())
        .map(|(i, pos)| Finding {
            number: WORDS_TO_NUMBER[i],
            pos: pos.unwrap(),
        })
        .collect::<Vec<_>>();
    let last_indices = WORDS
        .iter()
        .map(|w| s.rfind(w))
        .enumerate()
        .filter(|(_i, pos)| pos.is_some())
        .map(|(i, pos)| Finding {
            number: WORDS_TO_NUMBER[i],
            pos: pos.unwrap(),
        })
        .collect::<Vec<_>>();
    let first = first_indices.clone().into_iter().min_by_key(|&f| f.pos);
    let last = last_indices.into_iter().max_by_key(|&f| f.pos);
    (first, last)
}

fn pick_digit(of1: Option<Finding>, of2: Option<Finding>, ord: Ordering) -> u32 {
    match (of1, of2) {
        (Some(f1), Some(f2)) => {
            if f1.pos.cmp(&f2.pos) == ord {
                f1.number
            } else {
                f2.number
            }
        }
        (Some(f1), None) => f1.number,
        (None, Some(f2)) => f2.number,
        _ => panic!(),
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let r1 = lines
        .iter()
        .map(|l| {
            let c1 = l
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let c2 = l
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            c1 * 10 + c2
        })
        .sum::<u32>();
    println!("{}", r1);

    let r2 = lines
        .iter()
        .map(|l| {
            let first_digit = l.find(|c: char| char::is_ascii_digit(&c)).map(|i| Finding {
                pos: i,
                number: l.chars().nth(i).unwrap().to_digit(10).unwrap(),
            });
            let last_digit = l
                .rfind(|c: char| char::is_ascii_digit(&c))
                .map(|i| Finding {
                    pos: i,
                    number: l.chars().nth(i).unwrap().to_digit(10).unwrap(),
                });
            let (first_word, last_word) = find_words(l);
            let c1 = pick_digit(first_digit, first_word, Ordering::Less);
            let c2 = pick_digit(last_digit, last_word, Ordering::Greater);
            c1 * 10 + c2
            // c1 * 10 + c2
        })
        .sum::<u32>();
    println!("{}", r2);
}
