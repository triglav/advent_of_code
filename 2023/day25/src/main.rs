use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

use itertools::Itertools;

fn parse(line: &str) -> (&str, Vec<&str>) {
    let t = line.split(':').collect::<Vec<&str>>();
    let left = t[0].trim();
    let right = t[1].split_whitespace().collect::<Vec<&str>>();
    (left, right)
}

fn score_edges<'a>(
    network: &'a HashMap<&'a str, HashSet<&'a str>>,
    start: &'a str,
    edges: &mut HashMap<(&'a str, &'a str), usize>,
) {
    let mut todo = VecDeque::from(vec![(start, vec![])]);
    let mut seen = HashMap::<&str, Vec<&str>>::new();

    while let Some((n, mut path)) = todo.pop_front() {
        path.push(n);
        if seen.contains_key(n) {
            continue;
        }
        seen.insert(n, path.clone());
        for m in network
            .get(n)
            .unwrap()
            .iter()
            .filter(|&m| !seen.contains_key(m))
        {
            todo.push_back((m, path.clone()));
        }
    }

    seen.into_iter().for_each(|(_, path)| {
        path.into_iter().tuple_windows().for_each(|(l, r)| {
            let a = std::cmp::min(l, r);
            let b = std::cmp::max(l, r);
            *edges.entry((a, b)).or_default() += 1;
        });
    });
}

fn trace_sub_network<'a>(
    network: &'a HashMap<&'a str, HashSet<&'a str>>,
    edges_to_remove: &'a HashSet<(&'a str, &'a str)>,
) -> HashSet<&'a str> {
    let start = network.iter().next().unwrap().0;

    let mut seen = HashSet::<&str>::new();
    let mut todo = VecDeque::from(vec![start]);

    while let Some(n) = todo.pop_front() {
        if seen.contains(n) {
            continue;
        }
        seen.insert(n);
        for m in network
            .get(n)
            .unwrap()
            .iter()
            .filter(|&m| !seen.contains(m))
        {
            let l = n;
            let r = m;
            let a = std::cmp::min(l, r);
            let b = std::cmp::max(l, r);
            let edge = (*a, *b);
            if !edges_to_remove.contains(&edge) {
                todo.push_back(m);
            }
        }
    }
    seen
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut network = HashMap::<&str, HashSet<&str>>::new();
    lines.iter().map(|l| parse(l)).for_each(|(l, r)| {
        r.iter().for_each(|n| {
            network.entry(n).or_default().insert(l);
        });
        network.entry(l).or_default().extend(r);
    });

    let mut edges = HashMap::<(&str, &str), usize>::new();
    network.iter().for_each(|(n, _)| {
        score_edges(&network, n, &mut edges);
    });
    let mut edges = edges.into_iter().collect::<Vec<_>>();
    edges.sort_by_key(|e| e.1);
    edges.reverse();

    let edges_to_remove = edges
        .into_iter()
        .take(3)
        .map(|(e, _)| e)
        .collect::<HashSet<_>>();
    let sub_network = trace_sub_network(&network, &edges_to_remove);
    let r = sub_network.len() * (network.keys().len() - sub_network.len());
    println!("{}", r);
}
