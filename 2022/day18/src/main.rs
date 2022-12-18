use itertools::iproduct;
use std::{
    cmp::{max, min},
    collections::HashSet,
    io,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

fn parse_coord(s: &str) -> Coord {
    let v: Vec<_> = s.split(',').map(|t| t.parse::<i32>().unwrap()).collect();
    Coord {
        x: *v.get(0).unwrap(),
        y: *v.get(1).unwrap(),
        z: *v.get(2).unwrap(),
    }
}

type CoordSet = HashSet<Coord>;

fn get_adjacent(c: &Coord) -> Vec<Coord> {
    vec![
        Coord::new(c.x + 1, c.y, c.z),
        Coord::new(c.x - 1, c.y, c.z),
        Coord::new(c.x, c.y + 1, c.z),
        Coord::new(c.x, c.y - 1, c.z),
        Coord::new(c.x, c.y, c.z + 1),
        Coord::new(c.x, c.y, c.z - 1),
    ]
}

fn count_adjacent(set: &CoordSet, c: &Coord) -> usize {
    get_adjacent(c)
        .into_iter()
        .filter(|c| set.contains(c))
        .count()
}

fn is_within_boundaries(c: &Coord, (b_min, b_max): (Coord, Coord)) -> bool {
    c.x >= b_min.x
        && c.x <= b_max.x
        && c.y >= b_min.y
        && c.y <= b_max.y
        && c.z >= b_min.z
        && c.z <= b_max.z
}

fn flood_fill(set: &mut CoordSet, start: Coord, boundaries: (Coord, Coord)) {
    let mut todo = Vec::<Coord>::new();
    todo.push(start);

    while let Some(c) = todo.pop() {
        if !set.contains(&c) {
            set.insert(c);
            let free_adjacent: Vec<_> = get_adjacent(&c)
                .into_iter()
                .filter(|c| is_within_boundaries(c, boundaries))
                .filter(|c| !set.contains(c))
                .collect();
            todo.extend(free_adjacent);
        }
    }
}

fn main() {
    let coords = io::stdin()
        .lines()
        .map(|l| parse_coord(l.unwrap().as_str()));

    let mut s = CoordSet::new();
    let mut r1 = 0;
    for c in coords {
        let a = count_adjacent(&s, &c);
        s.insert(c);
        r1 += 6 - 2 * a;
    }
    println!("{}", r1);

    let b_min = s
        .clone()
        .into_iter()
        .reduce(|a, b| Coord {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
            z: min(a.z, b.z),
        })
        .unwrap();
    let b_max = s
        .clone()
        .into_iter()
        .reduce(|a, b| Coord {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
            z: max(a.z, b.z),
        })
        .unwrap();
    let b = (b_min, b_max);

    for (x, y, z) in iproduct!([b_min.x, b_max.x], b_min.y..=b_max.y, b_min.z..=b_max.z) {
        flood_fill(&mut s, Coord::new(x, y, z), b)
    }
    for (x, y, z) in iproduct!(b_min.x..=b_max.x, [b_min.y, b_max.y], b_min.z..=b_max.z) {
        flood_fill(&mut s, Coord::new(x, y, z), b)
    }
    for (x, y, z) in iproduct!(b_min.x..=b_max.x, b_min.y..=b_max.y, [b_min.z, b_max.z]) {
        flood_fill(&mut s, Coord::new(x, y, z), b)
    }

    let inner: Vec<_> = iproduct!(b_min.x..=b_max.x, b_min.y..=b_max.y, b_min.z..=b_max.z)
        .filter_map(|(x, y, z)| {
            let c = Coord::new(x, y, z);
            if !s.contains(&c) {
                Some(c)
            } else {
                None
            }
        })
        .collect();
    let mut r2 = r1;
    for c in inner {
        let a = count_adjacent(&s, &c);
        s.insert(c);
        r2 += 6 - 2 * a;
    }
    println!("{}", r2);
}
