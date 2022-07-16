use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut c = 0;
    let mut test = HashSet::<u8>::new();
    for l in lines {
        if l.is_empty() {
            c += test.len();
            test.clear();
            continue;
        }
        for c in l.as_bytes() {
            test.insert(*c);
        }
    }
    c += test.len();
    println!("{}", c);
}
