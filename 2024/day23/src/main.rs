use std::{
    collections::{HashMap, HashSet},
    io,
};

fn find_groups<'a>(
    connections: &'a HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<(&'a str, &'a str, &'a str)> {
    let mut groups = HashSet::new();
    for (&n, e) in connections.iter() {
        for &n2 in e.iter() {
            for (&n3, e3) in connections.iter() {
                if n3 == n || n3 == n2 {
                    continue;
                }
                if e3.contains(n) && e3.contains(n2) {
                    let mut v = [n, n2, n3];
                    v.sort();
                    groups.insert((v[0], v[1], v[2]));
                }
            }
        }
    }
    groups
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let connections = lines
        .iter()
        .map(|l| l.split('-').collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .fold(HashMap::<&str, HashSet<&str>>::new(), |mut acc, (a, b)| {
            acc.entry(a).or_default().insert(b);
            acc.entry(b).or_default().insert(a);
            acc
        });
    let groups = find_groups(&connections);
    let r1 = groups
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count();
    println!("{}", r1);
}
