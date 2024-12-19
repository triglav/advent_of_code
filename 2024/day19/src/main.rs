use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

fn solve(towels: &Vec<&str>, design: &str) -> HashSet<String> {
    // println!("\n{}", design);
    let mut todo = VecDeque::from(
        towels
            .iter()
            .map(|t| (t.to_string(), t.to_string()))
            .collect::<Vec<_>>(),
    );
    let mut solutions = HashSet::new();
    let mut seen = HashSet::new();
    while let Some((d, p)) = todo.pop_front() {
        if d == design {
            solutions.insert(p.clone());
            // println!("solution: {} // {}", p, solutions.len());
            continue;
        }
        if d.len() > design.len() {
            continue;
        }
        if !design.starts_with(d.as_str()) {
            continue;
        }
        if seen.contains(p.as_str()) {
            // println!("seen: {}", p);
            continue;
        }
        // println!("{}", p);
        seen.insert(p.clone());
        for &t in towels.iter() {
            let d2 = format!("{}{}", d, t);
            if design.starts_with(d2.as_str()) {
                let p2 = format!("{}-{}", p, t);
                todo.push_front((d2, p2));
            }
        }
    }
    // println!("{:?}", seen);
    // println!(" -> {:?}", solutions);
    // println!(" -> {}", solutions.len());
    solutions
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let towels = lines[0].split(",").map(|x| x.trim()).collect::<Vec<_>>();
    let designs = lines[2..].iter().collect::<Vec<_>>();

    let towel_combos = towels
        .iter()
        .map(|t| (t.to_string(), solve(&towels, t)))
        .collect::<HashMap<_, _>>();
    println!("{:?}", towel_combos);

    let solutions = designs
        .iter()
        .map(|d| solve(&towels, d))
        .collect::<Vec<_>>();
    let r1 = solutions.iter().filter(|&s| !s.is_empty()).count();
    println!("{}", r1);

    let r2 = solutions
        .iter()
        // .inspect(|z| println!("solution: {:?}", z))
        .map(|s| {
            s.iter()
                // .inspect(|z| println!("* {}", z))
                .map(|solution| {
                    solution
                        .split('-')
                        .map(|ss| towel_combos.get(ss).map_or(0, |s| s.len()))
                        // .inspect(|z| println!("  -> {}", z))
                        .product::<usize>()
                })
                .sum::<usize>()
        })
        .collect::<Vec<_>>();
    println!("{:?}", r2);
}
