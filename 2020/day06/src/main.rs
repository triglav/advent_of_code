use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    let mut r = 0;
    let mut test = HashSet::<u8>::new();

    let mut r2 = 0;
    let mut test2 = HashMap::<u8, u32>::new();
    let mut p = 0;

    for l in lines {
        if l.is_empty() {
            r += test.len();
            test.clear();

            r2 += test2.values().filter(|c| **c == p).count();
            p = 0;
            test2.clear();
            continue;
        }
        p += 1;
        for c in l.as_bytes() {
            test.insert(*c);
            match test2.get_mut(c) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    test2.insert(*c, 1);
                }
            };
        }
    }
    r += test.len();
    r2 += test2.values().filter(|c| **c == p).count();
    println!("{}", r);
    println!("{}", r2);
}
