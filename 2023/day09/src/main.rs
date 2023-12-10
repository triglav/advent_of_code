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

fn extrapolate(h: Vec<i32>) -> (i32, i32) {
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
    let r1 = *differences[0].last().unwrap();

    differences.last_mut().unwrap().insert(0, 0);
    for i in (0..differences.len() - 1).rev() {
        let d = differences[i + 1][0];
        let b = differences[i][0];
        let a = b - d;
        differences[i].insert(0, a);
    }
    let r2 = differences[0][0];

    (r1, r2)
}

fn main() {
    let histories = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap()))
        .map(extrapolate)
        .collect_vec();
    let r1 = histories.iter().map(|(r1, _r2)| r1).sum::<i32>();
    println!("{}", r1);

    let r2 = histories.into_iter().map(|(_r1, r2)| r2).sum::<i32>();
    println!("{}", r2);
}
