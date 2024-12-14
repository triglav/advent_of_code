use std::{collections::HashSet, io};

use itertools::{iproduct, Itertools};
use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

fn print(robots: &HashSet<(i64, i64)>, size: Coords) {
    iproduct!(0..size.y, 0..size.x).for_each(|(y, x)| {
        if robots.contains(&(x, y)) {
            print!("X");
        } else {
            print!(".");
        }
        if x == size.x - 1 {
            println!()
        }
    });
    println!();
}

fn detect_tree(robots: &[(Coords, Coords)]) -> bool {
    let threshold = 10;
    robots
        .iter()
        .map(|(p, _v)| (p.x, p.y))
        .sorted_by_key(|&(_x, y)| y)
        .chunk_by(|&(_x, y)| y)
        .into_iter()
        .map(|(_y, g)| g.collect_vec())
        .filter(|v| v.len() > threshold)
        .any(|row| {
            let consecutive = row
                .iter()
                .map(|&(x, _y)| x)
                .sorted()
                .fold(vec![], |mut acc, c| {
                    if acc.is_empty() {
                        acc.push(vec![c]);
                    } else {
                        let last = acc.last().unwrap().last().unwrap();
                        if *last != c {
                            if last + 1 == c {
                                acc.last_mut().unwrap().push(c);
                            } else {
                                acc.push(vec![c]);
                            }
                        }
                    }
                    acc
                });
            consecutive.into_iter().any(|c| c.len() > threshold)
        })
}

fn solve2(robots: &[(Coords, Coords)], size: Coords, show_tree: bool) -> Option<i64> {
    let mut robots = Vec::from(robots);
    for i in 1..(size.x * size.y) {
        robots = robots
            .into_iter()
            .map(|(rpos, rvel)| (find_pos(rpos, rvel, size, 1), rvel))
            .collect::<Vec<_>>();
        if detect_tree(&robots) {
            if show_tree {
                let hashset = robots
                    .iter()
                    .map(|(p, _v)| (p.x, p.y))
                    .collect::<HashSet<_>>();
                print(&hashset, size);
            }
            return Some(i);
        }
    }
    None
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

    let show_tree = false;
    let r2 = solve2(&robots, size, show_tree).unwrap();
    println!("{}", r2);
}
