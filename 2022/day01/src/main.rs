use std::{cmp::max, io};

fn main() {
    let mut r: u32 = 0;
    let mut c: u32 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(t) => match t.parse::<u32>() {
                Ok(n) => c += n,
                Err(_) => {
                    r = max(r, c);
                    c = 0;
                }
            },
            Err(_) => panic!("Invalid input"),
        }
    }
    println!("{}", r);
}
