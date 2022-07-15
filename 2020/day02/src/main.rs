use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let r = lines
        .iter()
        .filter(|l| {
            let m = re.captures(l).unwrap();
            let b = m[1].parse::<usize>().unwrap();
            let t = m[2].parse::<usize>().unwrap();
            let c = &m[3];
            let p = &m[4];
            let count = p.matches(c).count();
            count >= b && count <= t
        })
        .count();
    println!("{}", r);
    let r = lines
        .iter()
        .filter(|l| {
            let m = re.captures(l).unwrap();
            let l1 = m[1].parse::<usize>().unwrap();
            let l2 = m[2].parse::<usize>().unwrap();
            let c = m[3].as_bytes()[0] as char;
            let p = &m[4];
            let b1 = p.as_bytes()[l1 - 1] as char == c;
            let b2 = p.as_bytes()[l2 - 1] as char == c;
            b1 != b2
        })
        .count();
    println!("{}", r);
}
