use std::{cmp::max, io, collections::BinaryHeap};

fn main() {
    let mut r: u32 = 0;
    let mut r2 = BinaryHeap::new();
    let mut c: u32 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(t) => match t.parse::<u32>() {
                Ok(n) => c += n,
                Err(_) => {
                    r = max(r, c);
                    r2.push(c);
                    c = 0;
                }
            },
            Err(_) => panic!("Invalid input"),
        }
    }
    r = max(r, c);
    r2.push(c);
    println!("{}", r);
    println!("{}", r2.pop().unwrap() + r2.pop().unwrap() + r2.pop().unwrap());
}
