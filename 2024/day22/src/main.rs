use std::{collections::HashMap, io};

use itertools::{self, Itertools};

fn mix(n: usize, secret: usize) -> usize {
    n ^ secret
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn next_number(secret: usize) -> usize {
    let secret = prune(mix(secret * 64, secret));
    let secret = prune(mix(secret / 32, secret));
    prune(mix(secret * 2048, secret))
}

fn next_number_acc(secret: usize, count: usize) -> usize {
    let mut secret = secret;
    for _ in 0..count {
        secret = next_number(secret);
    }
    secret
}

fn price(secret: usize) -> usize {
    secret % 10
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let r1 = lines
        .iter()
        .map(|&n| next_number_acc(n, 2000))
        .sum::<usize>();
    println!("{}", r1);

    let r2 = lines
        .iter()
        .flat_map(|&secret| {
            (0..2000)
                .scan(secret, |secret, _| {
                    let prev_secret = *secret;
                    *secret = next_number(*secret);
                    Some(prev_secret)
                })
                .map(price)
                .tuple_windows::<(_, _)>()
                .map(|(a, b)| (b, b as isize - a as isize))
                .tuple_windows::<(_, _, _, _)>()
                .map(|(a, b, c, d)| {
                    let combo = (a.1, b.1, c.1, d.1);
                    let price = d.0;
                    (combo, price)
                })
                .unique_by(|&(combo, _)| combo)
        })
        .fold(HashMap::new(), |mut acc, (combo, price)| {
            acc.entry(combo)
                .and_modify(|e| *e += price)
                .or_insert(price);
            acc
        })
        .into_iter()
        .max_by_key(|&(_, price)| price)
        .unwrap()
        .1;
    println!("{}", r2);
}
