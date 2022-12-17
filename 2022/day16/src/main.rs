use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io,
};

#[derive(Clone)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<String>,
}

fn parse_valve<'a>(s: String) -> Valve {
    let t0: Vec<_> = s.split(';').collect();
    let t1: Vec<_> = t0[0].split(' ').collect();
    let name = t1[1];
    let rate = t1[4]
        .trim_matches(|c: char| !c.is_digit(10))
        .parse::<u32>()
        .unwrap();
    let tunnels: Vec<_> = t0[1]
        .trim_matches(|c: char| !c.is_uppercase())
        .split(',')
        .map(str::trim)
        .map(str::to_string)
        .collect();
    Valve {
        name: name.to_string(),
        rate,
        tunnels,
    }
}

fn measure_distances<'a>(
    valves: &'a HashMap<String, Valve>,
    pos: &'a str,
) -> HashMap<&'a str, u32> {
    let mut visited = HashMap::new();
    let mut todo = Vec::new();
    todo.push((pos, 0));
    while !todo.is_empty() {
        let (p, d) = todo.pop().unwrap();
        if d > 0 {
            visited.insert(p, d);
        }
        let valve = valves.get(p).unwrap();
        for v in valve.tunnels.iter().filter(|v| {
            if let Some(v2) = visited.get(v.as_str()) {
                *v2 > d + 1
            } else {
                true
            }
        }) {
            todo.push((v, d + 1));
        }
    }
    visited
}

struct Result<'a> {
    open_valves: HashSet<&'a str>,
    flow: u32,
}

fn traverse<'a>(
    v0: &'a Valve,
    time: u32,
    open_valves: HashSet<&'a str>,
    closed_valves: HashSet<&'a str>,
    flow: u32,
    valves: &'a HashMap<String, Valve>,
    distances: &HashMap<&str, HashMap<&str, u32>>,
) -> Vec<Result<'a>> {
    if time <= 0 {
        let r = vec![Result {
            open_valves: open_valves,
            flow,
        }];
        return r;
    }

    let mut results = vec![];

    for name2 in closed_valves.iter() {
        assert_ne!(v0.name, *name2);

        let v2 = valves.get(*name2).unwrap();
        assert_ne!(v2.rate, 0);

        // +1 to open
        let distance = *distances.get(v0.name.as_str()).unwrap().get(name2).unwrap() + 1;
        if distance > time {
            continue;
        }

        let mut open_valves2 = open_valves.clone();
        open_valves2.insert(name2);

        let mut closed_valves2 = closed_valves.clone();
        closed_valves2.remove(name2);

        let t2 = time - distance;
        let f2 = flow + t2 * v2.rate;
        let r = traverse(&v2, t2, open_valves2, closed_valves2, f2, valves, distances);
        results.extend(r);
    }
    if results.is_empty() {
        results.push(Result { open_valves, flow });
    }
    results
}

fn paths_cross(a: &Result, b: &Result) -> bool {
    a.open_valves.iter().any(|v| b.open_valves.contains(*v))
}

fn find_best(paths: &Vec<Result>) -> u32 {
    let mut best = 0;
    for i in 0..(paths.len() - 1) {
        let p = paths.get(i).unwrap();
        for i2 in (i + 1)..paths.len() {
            let p2 = paths.get(i2).unwrap();
            if !paths_cross(p, p2) {
                best = max(best, p.flow + p2.flow);
                break;
            }
        }
    }
    best
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let valves: HashMap<_, _> = lines
        .into_iter()
        .map(parse_valve)
        .map(|v| (v.name.clone(), v))
        .collect();

    let distances: HashMap<_, _> = valves
        .keys()
        .map(|r| {
            let distances = measure_distances(&valves, r);
            (r.as_str(), distances)
        })
        .collect();

    let start = valves.get("AA").unwrap();
    let valves_with_flow: HashSet<_> = valves
        .iter()
        .filter(|(_name, v)| v.rate > 0)
        .map(|(name, _v)| name.as_str())
        .collect();
    let paths = traverse(
        start,
        30,
        HashSet::new(),
        valves_with_flow.clone(),
        0,
        &valves,
        &distances,
    );
    let r1 = paths.iter().max_by_key(|r| r.flow).unwrap().flow;
    println!("{}", r1);

    let mut paths2 = traverse(
        start,
        26,
        HashSet::new(),
        valves_with_flow.clone(),
        0,
        &valves,
        &distances,
    );
    paths2.sort_by_key(|r| r.flow);
    paths2.reverse();
    let r2 = find_best(&paths2);
    println!("{}", r2);
}
