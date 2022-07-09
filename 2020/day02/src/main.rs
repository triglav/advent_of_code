use std::io::{self, BufRead};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();
    let r = stdin
        .lock()
        .lines()
        .filter(|l| {
            let s = &l.as_ref().unwrap();
            let m = re.captures(s).unwrap();
            let b = m[1].parse::<usize>().unwrap();
            let t = m[2].parse::<usize>().unwrap();
            let c = &m[3];
            let p = &m[4];
            let count = p.matches(c).count();
            count >= b && count <= t
        })
        .count();
    println!("{}", r);
}
