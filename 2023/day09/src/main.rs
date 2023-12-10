use std::io;

use itertools::Itertools;

fn parse(line: String) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn get_differences(h: &[i32]) -> Vec<i32> {
    h.windows(2).map(|w| w[1] - w[0]).collect_vec()
}

fn extrapolate(h: Vec<i32>) -> i32 {
    let mut differences = vec![];
    differences.push(h);
    loop {
        let d = differences.last().unwrap();
        // println!("{:?}", d);
        if d.iter().all(|&x| x == 0) {
            break;
        }
        let d2 = get_differences(d);
        differences.push(d2);
    }

    differences.last_mut().unwrap().push(0);

    for i in (0..differences.len() - 1).rev() {
        let d = *differences[i + 1].last().unwrap();
        let a = *differences[i].last().unwrap();
        let b = a + d;
        differences[i].push(b);
    }
    *differences[0].last().unwrap()
}

fn main() {
    let r1 = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap()))
        .map(extrapolate)
        .sum::<i32>();
    println!("{}", r1);
}
