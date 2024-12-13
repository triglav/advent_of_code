use std::{
    io,
    ops::{Add, Div, Mul, Sub},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: u64,
    y: u64,
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

impl Mul<u64> for Coords {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<u64> for Coords {
    type Output = Self;

    fn div(self, rhs: u64) -> Self::Output {
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
            x: m[1].parse::<u64>().unwrap(),
            y: m[2].parse::<u64>().unwrap(),
        })
        .unwrap()
}

fn parse_prize(s: &str) -> Coords {
    let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    re.captures(s)
        .map(|m| Coords {
            x: m[1].parse::<u64>().unwrap(),
            y: m[2].parse::<u64>().unwrap(),
        })
        .unwrap()
}

// 3 tokens button A
// 1 token button B
fn solve1(a: Coords, b: Coords, p: Coords) -> Option<u64> {
    let a_count = p / a;
    let max_a_count = a_count.x.min(a_count.y).min(100);

    (0..=max_a_count)
        .filter_map(|c_a| {
            let p2 = p - (a * c_a);
            let c_b = p2 / b;
            if c_b.x != c_b.y {
                return None;
            }
            let c_b = c_b.x;
            let p2 = a * c_a + b * c_b;
            if p2.x != p.x || p2.y != p.y {
                return None;
            }
            Some((c_a, c_b))
        })
        .map(|(a, b)| 3 * a + b)
        .min()
}

fn main() {
    let input = io::stdin().lines().map(|l| l.unwrap());
    let r1 = input
        .chunks(4)
        .into_iter()
        .filter_map(|chunk| {
            let c = chunk.collect_vec();
            let a = parse_button(&c[0]);
            let b = parse_button(&c[1]);
            let p = parse_prize(&c[2]);
            solve1(a, b, p)
        })
        .sum::<u64>();
    println!("{}", r1);
}
