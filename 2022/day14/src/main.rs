use std::{
    cmp::{max, min},
    collections::HashSet,
    io,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

type Structure = Vec<Coord>;

fn read_coord(s: &str) -> Coord {
    let t: Vec<_> = s.split(',').collect();
    Coord {
        x: t.get(0).unwrap().parse::<i32>().unwrap(),
        y: t.get(1).unwrap().parse::<i32>().unwrap(),
    }
}

fn read_structure(s: &String) -> Structure {
    s.split("->").map(|t| read_coord(t.trim())).collect()
}

type Cave = HashSet<Coord>;

fn direction(c0: Coord, c1: Coord) -> Coord {
    Coord {
        x: (c1.x - c0.x).signum(),
        y: (c1.y - c0.y).signum(),
    }
}

fn build_structure_part(cave: &mut Cave, c0: Coord, c1: Coord) {
    let d = direction(c0, c1);
    let mut p = c0;
    loop {
        cave.insert(p);

        if p.x == c1.x && p.y == c1.y {
            break;
        }

        p.x += d.x;
        p.y += d.y;
    }
}

fn build_structure(cave: &mut Cave, s: &Structure) {
    for part in s.windows(2) {
        build_structure_part(cave, part[0], part[1]);
    }
}

fn build_cave<'a, I>(iter: I) -> Cave
where
    I: Iterator<Item = &'a Structure>,
{
    let mut cave = Cave::new();
    for s in iter {
        build_structure(&mut cave, s);
    }
    cave
}

type Boundaries = (Coord, Coord);

fn get_boundaries<'a, I>(iter: I) -> Boundaries
where
    I: Iterator<Item = &'a Structure>,
{
    iter.flatten().fold(
        (Coord { x: 500, y: 0 }, Coord { x: 500, y: 0 }),
        |(c_min, c_max), c| {
            (
                Coord {
                    x: min(c_min.x, c.x),
                    y: min(c_min.y, c.y),
                },
                Coord {
                    x: max(c_max.x, c.x),
                    y: max(c_max.y, c.y),
                },
            )
        },
    )
}

fn simulate_sand_step(cave: &HashSet<Coord>, c0: Coord) -> Coord {
    let c1 = Coord {
        x: c0.x,
        y: c0.y + 1,
    };
    if !cave.contains(&c1) {
        return c1;
    }
    let c2 = Coord {
        x: c0.x - 1,
        y: c0.y + 1,
    };
    if !cave.contains(&c2) {
        return c2;
    }
    let c3 = Coord {
        x: c0.x + 1,
        y: c0.y + 1,
    };
    if !cave.contains(&c3) {
        return c3;
    }
    c0
}

fn is_within_boundaries(c0: Coord, b: Boundaries, floor: bool) -> bool {
    if floor {
        b.0.y <= b.0.y && b.1.y + 2 > c0.y
    } else {
        b.0.x <= c0.x && b.0.y <= b.0.y && b.1.x >= c0.x && b.1.y >= c0.y
    }
}

fn simulate_sand_unit(
    cave: &HashSet<Coord>,
    c0: Coord,
    b: Boundaries,
    floor: bool,
) -> Option<Coord> {
    let mut p = c0;
    loop {
        if floor && cave.contains(&c0) {
            break None;
        }
        let p2 = simulate_sand_step(cave, p);
        if p == p2 {
            break Some(p);
        }
        if !is_within_boundaries(p2, b, floor) {
            if floor {
                break Some(p);
            } else {
                break None;
            }
        }
        p = p2;
    }
}

fn simulate_sand(cave: &mut HashSet<Coord>, c0: Coord, b: Boundaries, floor: bool) -> usize {
    let mut c: usize = 0;
    loop {
        match simulate_sand_unit(cave, c0, b, floor) {
            Some(p) => {
                c += 1;
                cave.insert(p);
            }
            None => break,
        }
    }
    c
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap());
    let structures: Vec<_> = lines.map(|l| read_structure(&l)).collect();

    let cave0 = build_cave(structures.iter());
    let b = get_boundaries(structures.iter());
    let c0 = Coord { x: 500, y: 0 };

    let mut cave1 = cave0.clone();
    let r1 = simulate_sand(&mut cave1, c0, b, false);
    println!("{}", r1);

    let mut cave2 = cave0.clone();
    let r2 = simulate_sand(&mut cave2, c0, b, true);
    println!("{}", r2);
}
