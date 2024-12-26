use std::{
    collections::{BTreeSet, HashMap, HashSet},
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

fn find_largest_group<'a>(connections: &'a HashMap<&'a str, HashSet<&'a str>>) -> Vec<&'a str> {
    fn find_largest_group_rec<'a>(
        connections: &'a HashMap<&'a str, HashSet<&'a str>>,
        group: BTreeSet<&'a str>,
        seen: &mut HashSet<BTreeSet<&'a str>>,
    ) -> BTreeSet<&'a str> {
        if seen.contains(&group) {
            return group;
        }
        seen.insert(group.clone());
        let group_candidates = connections
            .iter()
            .filter(|&(n, _)| !group.contains(n))
            .collect::<Vec<_>>();
        if group_candidates.is_empty() {
            return group;
        }
        let largest_group = group_candidates
            .into_iter()
            .filter(|(_, e)| group.iter().all(|g| e.contains(g)))
            .map(|(n, _)| {
                let mut g2 = group.clone();
                g2.insert(*n);
                find_largest_group_rec(connections, g2, seen)
            })
            .max_by(|a, b| a.len().cmp(&b.len()));
        largest_group.unwrap_or(group)
    }

    let mut seen = HashSet::new();
    let mut largest_group = find_largest_group_rec(connections, BTreeSet::new(), &mut seen)
        .into_iter()
        .collect::<Vec<_>>();
    largest_group.sort();
    largest_group
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

    let r2 = find_largest_group(&connections).join(",");
    println!("{}", r2);
}
