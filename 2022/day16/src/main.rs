use std::{
    collections::{HashMap, HashSet},
    io,
};

#[derive(Clone)]
struct Valve {
    name: String,
    rate: i32,
    tunnels: Vec<String>,
}

fn parse_valve<'a>(s: String) -> Valve {
    let t0: Vec<_> = s.split(';').collect();
    let t1: Vec<_> = t0[0].split(' ').collect();
    let name = t1[1];
    let rate = t1[4]
        .trim_matches(|c: char| !c.is_digit(10))
        .parse::<i32>()
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
) -> HashMap<&'a str, i32> {
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

fn traverse<'a>(
    v0: &'a Valve,
    time: i32,
    open_valves: HashSet<&str>,
    closed_valves: HashSet<&str>,
    valves: &'a HashMap<String, Valve>,
    distances: &HashMap<&str, HashMap<&str, i32>>,
) -> i32 {
    if time <= 0 {
        return 0;
    }

    let rate0 = v0.rate * time;
    let mut max_rate = 0;
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
        // let r2 = rate + t2 * v2.rate;
        let r = traverse(&v2, t2, open_valves2, closed_valves2, valves, distances);
        if r > max_rate {
            max_rate = r;
        }
    }
    rate0 + max_rate
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
    let r1 = traverse(
        start,
        30,
        HashSet::new(),
        valves_with_flow.clone(),
        &valves,
        &distances,
    );
    println!("{}", r1);
}
