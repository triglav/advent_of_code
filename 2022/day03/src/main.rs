use std::{collections::HashSet, io};

fn item_priority(i: char) -> u32 {
    let c = i as u32;
    if c >= 'a' as u32 {
        c - 'a' as u32 + 1
    } else {
        c - 'A' as u32 + 27
    }
}

fn find_common_items(a: &str, b: &str) -> HashSet<char> {
    let r1 = a.chars().fold(HashSet::new(), |mut a, c| {
        a.insert(c);
        a
    });
    b.chars().fold(HashSet::new(), |mut a, c| {
        if r1.contains(&c) {
            a.insert(c);
        }
        a
    })
}

fn main() {
    let r1 = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            find_common_items(a, b)
                .iter()
                .map(|xx| item_priority(*xx))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", r1);
}
