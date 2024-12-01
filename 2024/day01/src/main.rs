use std::{collections::HashMap, io};

fn parse(line: &str) -> (u32, u32) {
    let t = line
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    (t[0], t[1])
}

fn main() {
    let (mut l1, mut l2) = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .fold((vec![], vec![]), |(mut l1, mut l2), (a, b)| {
            l1.push(a);
            l2.push(b);
            (l1, l2)
        });
    l1.sort();
    l2.sort();
    let r1 = l1.iter().zip(&l2).map(|(a, b)| a.abs_diff(*b)).sum::<u32>();
    println!("{}", r1);

    let right_hits = l2.iter().fold(HashMap::new(), |mut m, &b| {
        m.entry(b).and_modify(|e| *e += 1).or_insert(1);
        m
    });
    let r2 = l1
        .iter()
        .map(|a| a * right_hits.get(a).unwrap_or(&0))
        .sum::<u32>();
    println!("{:?}", r2);
}
