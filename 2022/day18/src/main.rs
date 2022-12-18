use std::{collections::HashSet, io};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
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

fn count_adjacent(set: &CoordSet, c: &Coord) -> usize {
    fn check(set: &CoordSet, x: i32, y: i32, z: i32) -> usize {
        if set.contains(&Coord { x, y, z }) {
            1
        } else {
            0
        }
    }

    let mut r = 0;
    r += check(set, c.x + 1, c.y, c.z);
    r += check(set, c.x - 1, c.y, c.z);
    r += check(set, c.x, c.y + 1, c.z);
    r += check(set, c.x, c.y - 1, c.z);
    r += check(set, c.x, c.y, c.z + 1);
    r += check(set, c.x, c.y, c.z - 1);
    r
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
}
