use std::collections::{HashMap, VecDeque};
use std::ops::{Add, Div, Mul, Sub};
use std::{fmt, io};

use itertools::{iproduct, Itertools};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

impl fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

#[derive(Clone)]
struct Grid<T> {
    pub width: i64,
    pub height: i64,
    pub tiles: Vec<T>,
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display + Copy + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        iproduct!(0..self.height, 0..self.width).try_for_each(|(y, x)| {
            let t = self.tiles[(y * self.width + x) as usize];
            write!(f, "{}", t)?;
            if x == self.width - 1 {
                writeln!(f)?;
            }
            Ok(())
        })
    }
}

impl<T> Grid<T>
where
    T: Copy + Default + PartialEq,
{
    pub fn new(width: i64, height: i64, default: T) -> Grid<T> {
        Grid {
            width,
            height,
            tiles: vec![default; (width * height) as usize],
        }
    }

    pub fn get(&self, p: Coords) -> T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles[(p.y * self.width + p.x) as usize]
    }

    pub fn get_mut(&mut self, p: Coords) -> &mut T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles
            .get_mut((p.y * self.width + p.x) as usize)
            .unwrap()
    }

    pub fn is_valid(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_neighbours(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|&(x, y)| self.is_valid(x, y))
            .collect()
    }
}

fn parse(s: &str) -> Coords {
    let v = s
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    Coords { x: v[0], y: v[1] }
}

fn solve(memory: &Grid<char>) -> Option<i32> {
    let start = Coords { x: 0, y: 0 };
    let end = Coords {
        x: memory.width - 1,
        y: memory.height - 1,
    };

    let mut visited = HashMap::new();
    let mut todo = VecDeque::from([(start, 0)]);
    while let Some((p, steps)) = todo.pop_front() {
        if memory.get(p) != '.' {
            continue;
        }
        if let Some(&steps2) = visited.get(&p) {
            if steps2 <= steps {
                continue;
            }
        }
        visited.insert(p, steps);
        memory.get_neighbours(p.x, p.y).iter().for_each(|&(x, y)| {
            todo.push_back((Coords { x, y }, steps + 1));
        });
    }
    visited.get(&end).copied()
}

fn main() {
    let bytes = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();

    let memory_dimension = 71;
    let bytes_count = 1024;
    let mut memory = Grid::new(memory_dimension, memory_dimension, '.');
    bytes
        .iter()
        .take(bytes_count)
        .for_each(|&p| *memory.get_mut(p) = '#');

    let r1 = solve(&memory).unwrap();
    println!("{}", r1);

    let r2_i = (0..bytes.len()).collect_vec().partition_point(|&i| {
        let mut memory = Grid::new(memory_dimension, memory_dimension, '.');
        bytes.iter().take(i).for_each(|&p| *memory.get_mut(p) = '#');
        solve(&memory).is_some()
    });
    let r2 = bytes[r2_i - 1];
    println!("{},{}", r2.x, r2.y);
}
