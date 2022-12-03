use itertools::Itertools;
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

fn note_items(items: &String) -> Vec<i32> {
    items.chars().fold(vec![0; 52], |mut a, c| {
        a[(item_priority(c) - 1) as usize] = 1;
        a
    })
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect_vec();

    let r1 = lines
        .iter()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            find_common_items(a, b)
                .iter()
                .map(|xx| item_priority(*xx))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", r1);

    let chunks = &lines.into_iter().chunks(3);
    let r2 = chunks.into_iter().map(|chunks| {
        let zz = chunks
            .map(|c| note_items(&c))
            .reduce(|a, b| {
                a.iter()
                    .zip(b.iter())
                    .map(|(ia, ib)| *ia + *ib)
                    .collect_vec()
            })
            .unwrap();
        (zz.iter().position(|i| *i == 3).unwrap() + 1) as u32
    }).sum::<u32>();

    println!("{}", r2);
}
