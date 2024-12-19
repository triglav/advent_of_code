use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn solve(towels: &Vec<&str>, design: &str) -> usize {
    let mut todo = VecDeque::from(towels.iter().map(|t| t.to_string()).collect::<Vec<_>>());
    let mut solutions = vec![];
    let mut seen = HashSet::new();
    while let Some(d) = todo.pop_front() {
        if d == design {
            solutions.push(d);
            continue;
        }
        if d.len() > design.len() {
            continue;
        }
        if !design.starts_with(d.as_str()) {
            continue;
        }
        if seen.contains(d.as_str()) {
            continue;
        }
        seen.insert(d.clone());
        for &t in towels.iter() {
            let ss = format!("{}{}", d, t);
            if design.starts_with(ss.as_str()) {
                todo.push_back(ss);
            }
        }
    }
    solutions.len()
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let towels = lines[0].split(",").map(|x| x.trim()).collect::<Vec<_>>();
    let designs = lines[2..].iter().collect::<Vec<_>>();

    let r1 = designs
        .iter()
        .map(|d| solve(&towels, d))
        .filter(|&x| x > 0)
        .count();
    println!("{:?}", r1);
}
