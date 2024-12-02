use std::io;

fn parse(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn check_increasing(v: &[i32]) -> Vec<usize> {
    let mut err = vec![];
    for i in 1..v.len() {
        let d = v[i] - v[i - 1];
        if !(1..=3).contains(&d) {
            err.push(i - 1);
            err.push(i);
        }
    }
    err
}

fn check_decreasing(v: &[i32]) -> Vec<usize> {
    let mut err = vec![];
    for i in 1..v.len() {
        let d = v[i - 1] - v[i];
        if !(1..=3).contains(&d) {
            err.push(i - 1);
            err.push(i);
        }
    }
    err
}

fn check1(v: &[i32]) -> bool {
    check_increasing(v).is_empty() || check_decreasing(v).is_empty()
}

fn check2(v: &[i32]) -> bool {
    let b1 = check_increasing(v);
    if b1.is_empty() {
        return true;
    }
    let b2 = check_decreasing(v);
    if b2.is_empty() {
        return true;
    }
    for i in b1 {
        let mut v = v.to_vec();
        v.remove(i);
        if check_increasing(&v).is_empty() {
            return true;
        }
    }
    for i in b2 {
        let mut v = v.to_vec();
        v.remove(i);
        if check_decreasing(&v).is_empty() {
            return true;
        }
    }
    false
}

fn main() {
    let r = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();
    let r1 = r.iter().filter(|a| check1(a)).count();
    println!("{}", r1);

    let r2 = r.iter().filter(|a| check2(a)).count();
    println!("{}", r2);
}
