use std::{collections::HashMap, io};

fn solve(towels: &Vec<&str>, design: &str) -> usize {
    let mut seen: HashMap<String, usize> = HashMap::new();
    fn solve_rec(towels: &[&str], design: &str, seen: &mut HashMap<String, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }
        if seen.contains_key(design) {
            return *seen.get(design).unwrap();
        }
        let c = towels
            .iter()
            .filter(|&&t| design.starts_with(t))
            .map(|&t| solve_rec(towels, &design[t.len()..], seen))
            .sum();
        seen.insert(design.to_string(), c);
        c
    }
    solve_rec(towels, design, &mut seen)
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let towels = lines[0].split(",").map(|x| x.trim()).collect::<Vec<_>>();
    let designs = lines[2..].iter().collect::<Vec<_>>();

    let solutions = designs
        .iter()
        .map(|d| solve(&towels, d))
        .collect::<Vec<_>>();

    let r1 = solutions.iter().filter(|&&x| x > 0).count();
    println!("{}", r1);

    let r2 = solutions.into_iter().sum::<usize>();
    println!("{}", r2);
}
