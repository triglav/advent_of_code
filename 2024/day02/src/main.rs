use std::io;

fn parse(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn check1(v: &[i32]) -> bool {
    let increasing = v[0] - v[1] < 0;
    if increasing {
        for i in 1..v.len() {
            let d = v[i] - v[i - 1];
            if !(1..=3).contains(&d) {
                return false;
            }
        }
        return true;
    }
    for i in 1..v.len() {
        let d = v[i - 1] - v[i];
        if !(1..=3).contains(&d) {
            return false;
        }
    }
    true
}

fn main() {
    let zz = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .filter(|a| check1(a))
        .count();
    println!("{:?}", zz);
}
