use std::io;

use regex::Regex;

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let r1 = re
        .captures_iter(&input)
        .map(|m| {
            let a = m[1].parse::<i32>().unwrap();
            let b = m[2].parse::<i32>().unwrap();
            a * b
        })
        .sum::<i32>();
    println!("{}", r1);

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let r2 = re_do
        .split(&input)
        .flat_map(|s| re_dont.split(s).take(1))
        .flat_map(|s| {
            re.captures_iter(s).map(|m| {
                let a = m[1].parse::<i32>().unwrap();
                let b = m[2].parse::<i32>().unwrap();
                a * b
            })
        })
        .sum::<i32>();
    println!("{:?}", r2);
}
