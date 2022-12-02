use itertools::Itertools;
use std::io;

fn keep_max3(mut v: Vec<u32>, i: u32) -> Vec<u32> {
    let idx = v.binary_search(&i).unwrap_or_else(|idx| idx);
    v.insert(idx, i);
    if v.len() > 3 {
        v.remove(0);
    }
    v
}

fn main() {
    let top3 = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap_or(0))
        .coalesce(|a, b| if b > 0 { Ok(a + b) } else { Err((a, 0)) })
        .fold(Vec::<u32>::with_capacity(4), keep_max3);

    let r1 = top3.last().unwrap();
    println!("{}", r1);

    let r2 = top3.iter().sum::<u32>();
    println!("{}", r2);
}
