use std::io;

use regex::Regex;

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap());

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    //
    let r1 = lines
        .flat_map(|l| {
            re.captures_iter(&l)
                .map(|m| {
                    let a = m[1].parse::<i32>().unwrap();
                    let b = m[2].parse::<i32>().unwrap();
                    a * b
                })
                .collect::<Vec<_>>()
        })
        .sum::<i32>();
    println!("{}", r1);
}
