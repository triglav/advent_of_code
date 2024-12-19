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
        if seen.contains(d.as_str()) {
            continue;
        }
        seen.insert(d.clone());
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

fn combos(towel_combos: &HashMap<String, HashSet<String>>, solution: &str) -> HashSet<String> {
    let mut cs = HashSet::new();
    let mut todo = vec![("".to_string(), solution.split('-').collect::<Vec<_>>())];
    while let Some((pattern, mut solution)) = todo.pop() {
        if let Some(t) = solution.pop() {
            if let Some(tc) = towel_combos.get(t) {
                tc.iter().for_each(|towel| {
                    let pattern2 = if pattern.is_empty() {
                        towel.to_string()
                    } else {
                        format!("{}-{}", towel, pattern)
                    };
                    todo.push((pattern2, solution.clone()));
                });
            }
        } else {
            cs.insert(pattern.to_string());
        }
    }
    // println!("{:?}", cs);
    cs
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut towels = lines[0].split(",").map(|x| x.trim()).collect::<Vec<_>>();
    towels.sort_by_key(|a| a.len());
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

    // let r2 = solutions
    //     .iter()
    //     // .inspect(|z| println!("solution: {:?}", z))
    //     .map(|s| {
    //         s.iter()
    //             // .inspect(|z| println!("* {}", z))
    //             .map(|solution| {
    //                 solution
    //                     .split('-')
    //                     .map(|ss| towel_combos.get(ss).map_or(0, |s| s.len()))
    //                     // .inspect(|z| println!("  -> {}", z))
    //                     .product::<usize>()
    //             })
    //             .sum::<usize>()
    //     })
    //     .collect::<Vec<_>>();
    let r2 = solutions
        .iter()
        .inspect(|solutions| println!("solution: {:?}", solutions))
        .map(|solutions| {
            solutions
                .iter()
                .map(|s| combos(&towel_combos, s))
                // .inspect(|c| println!("  -> {}", c))
                .fold(HashSet::new(), |acc, c| acc.union(&c).cloned().collect())
            // s.iter().inspect(|z| println!("* {}", z))
            // .map(|solution| {
            //     solution
            //         .split('-')
            //         .map(|ss| towel_combos.get(ss).map_or(0, |s| s.len()))
            //         // .inspect(|z| println!("  -> {}", z))
            //         .product::<usize>()
            // })
            // .sum::<usize>()
        })
        .inspect(|c| println!("  -> {:?} ({})", c, c.len()))
        .collect::<Vec<_>>();
    println!("{:?}", r2);
}
