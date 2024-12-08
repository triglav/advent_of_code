use std::{cmp::Ordering, collections::HashMap, io};

fn check(rules: &HashMap<u32, Vec<u32>>, updates: &[u32]) -> bool {
    (0..updates.len() - 1).all(|i| {
        let v0 = updates[i];
        let v1 = updates[i + 1];

        if let Some(r) = rules.get(&v0) {
            r.contains(&v1)
        } else {
            false
        }
    })
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut iter = lines.split(|l| l.is_empty());
    let rules = iter
        .next()
        .unwrap()
        .iter()
        .map(|l| {
            // 47|53
            let n = l
                .split("|")
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (n[0], n[1])
        })
        .fold(HashMap::new(), |mut acc, (a, b)| {
            let item: &mut Vec<u32> = acc.entry(a).or_default();
            item.push(b);
            acc
        });
    let updates = iter
        .next()
        .unwrap()
        .iter()
        .map(|l| {
            l.split(",")
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert!(iter.next().is_none());

    let r1 = updates
        .iter()
        .filter(|u| check(&rules, u))
        .map(|u| u[u.len() / 2])
        .sum::<u32>();
    println!("{}", r1);

    let r2 = updates
        .iter()
        .filter(|u| !check(&rules, u))
        .map(|u| {
            let mut u = u.clone();
            u.sort_by(|a, b| {
                if let Some(r_a) = rules.get(a) {
                    if r_a.contains(b) {
                        return Ordering::Less;
                    }
                }
                Ordering::Greater
            });
            assert!(check(&rules, &u));
            u
        })
        .map(|u| u[u.len() / 2])
        .sum::<u32>();
    println!("{}", r2);
}
