use std::io;

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
}
