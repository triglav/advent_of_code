use std::io;

use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64,
}

fn parse(s: &str) -> (Coords, Coords) {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures(s)
        .map(|m| {
            (
                Coords {
                    x: m[1].parse::<i64>().unwrap(),
                    y: m[2].parse::<i64>().unwrap(),
                },
                Coords {
                    x: m[3].parse::<i64>().unwrap(),
                    y: m[4].parse::<i64>().unwrap(),
                },
            )
        })
        .unwrap()
}

fn normalise(p: i64, s: i64) -> i64 {
    let mut p = p;
    while p < 0 {
        p += s;
    }
    p % s
}

fn find_pos(rpos: Coords, rvel: Coords, size: Coords, t: i64) -> Coords {
    Coords {
        x: normalise(rpos.x + rvel.x * t, size.x),
        y: normalise(rpos.y + rvel.y * t, size.y),
    }
}

fn pick_quadrant(pos: Coords, size: Coords) -> Option<usize> {
    let x1 = pos.x < size.x / 2;
    let x2 = pos.x > size.x / 2;

    let y1 = pos.y < size.y / 2;
    let y2 = pos.y > size.y / 2;

    match (x1, x2, y1, y2) {
        (true, false, true, false) => Some(0),
        (false, true, true, false) => Some(1),
        (true, false, false, true) => Some(2),
        (false, true, false, true) => Some(3),
        _ => None,
    }
}

fn main() {
    let robots = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();
    let t = 100;
    let size = Coords { x: 101, y: 103 };
    let r1 = robots
        .iter()
        .map(|&(rpos, rvel)| find_pos(rpos, rvel, size, t))
        .filter_map(|rpos| pick_quadrant(rpos, size))
        .fold(vec![0, 0, 0, 0], |mut acc, c| {
            acc[c] += 1;
            acc
        })
        .into_iter()
        .product::<u64>();
    println!("{}", r1);
}
