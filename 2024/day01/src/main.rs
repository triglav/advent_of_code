use std::io;

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
}
