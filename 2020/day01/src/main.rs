use std::io::{self, BufRead};

fn find_result(numbers: &Vec<u32>) -> Option<u32> {
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return Some(numbers[i] * numbers[j]);
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<u32> = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect();
    println!("{}", find_result(&numbers).unwrap());
}
