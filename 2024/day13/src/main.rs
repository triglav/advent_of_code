use std::{
    io,
    ops::{Add, Div, Mul, Sub},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: i64,
    y: i64,
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Div for Coords {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul for Coords {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i64> for Coords {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i64> for Coords {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

fn parse_button(s: &str) -> Coords {
    let re = Regex::new(r"Button [AB]: X\+(\d+), Y\+(\d+)").unwrap();
    re.captures(s)
        .map(|m| Coords {
            x: m[1].parse::<i64>().unwrap(),
            y: m[2].parse::<i64>().unwrap(),
        })
        .unwrap()
}

fn parse_prize(s: &str) -> Coords {
    let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    re.captures(s)
        .map(|m| Coords {
            x: m[1].parse::<i64>().unwrap(),
            y: m[2].parse::<i64>().unwrap(),
        })
        .unwrap()
}

fn solve(a: Coords, b: Coords, p: Coords) -> Option<(i64, i64)> {
    let det = a.x * b.y - a.y * b.x;
    if det == 0 {
        println!("none");
        return None;
    }
    let aa = b.y * p.x - b.x * p.y;
    let a_c = aa / det;
    if aa % det == 0 && a_c >= 0 {
        let bb = a.x * p.y - a.y * p.x;
        let b_c = bb / det;
        if bb % det == 0 && b_c >= 0 {
            return Some((a_c, b_c));
        }
    }
    None
}

fn solve1(a: Coords, b: Coords, p: Coords) -> Option<i64> {
    if let Some((ac, bc)) = solve(a, b, p) {
        if ac <= 100 && bc <= 100 {
            return Some(3 * ac + bc);
        }
    }
    None
}

fn solve2(a: Coords, b: Coords, p: Coords) -> Option<i64> {
    let p = Coords {
        x: p.x + 10000000000000,
        y: p.y + 10000000000000,
    };
    if let Some((ac, bc)) = solve(a, b, p) {
        Some(3 * ac + bc)
    } else {
        None
    }
}

fn main() {
    let input = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .chunks(4)
        .into_iter()
        .map(|chunk| {
            let c = chunk.collect_vec();
            let a = parse_button(&c[0]);
            let b = parse_button(&c[1]);
            let p = parse_prize(&c[2]);
            (a, b, p)
        })
        .collect_vec();
    let r1 = input
        .iter()
        .filter_map(|&(a, b, p)| solve1(a, b, p))
        .sum::<i64>();
    println!("{}", r1);
    let r2 = input
        .iter()
        .filter_map(|&(a, b, p)| solve2(a, b, p))
        .sum::<i64>();
    println!("{}", r2);
}
