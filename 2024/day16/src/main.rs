use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use std::{fmt, io};

use itertools::iproduct;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::East => Direction::South,
    }
}

fn turn_left(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::West,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::East => Direction::North,
    }
}

fn direction_vector(d: Direction) -> Coords {
    match d {
        Direction::North => Coords::north(),
        Direction::South => Coords::south(),
        Direction::West => Coords::west(),
        Direction::East => Coords::east(),
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    pub fn north() -> Self {
        Coords { x: 0, y: -1 }
    }
    pub fn south() -> Self {
        Coords { x: 0, y: 1 }
    }
    pub fn west() -> Self {
        Coords { x: -1, y: 0 }
    }
    pub fn east() -> Self {
        Coords { x: 1, y: 0 }
    }
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
    pub fn from(grid: Vec<Vec<T>>) -> Grid<T> {
        let width = grid[0].len() as i64;
        let height = grid.len() as i64;
        let tiles = grid.into_iter().flatten().collect::<Vec<_>>();
        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, p: Coords) -> T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles[(p.y * self.width + p.x) as usize]
    }

    pub fn find(&self, t: T) -> Option<Coords> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, &x)| x == t)
            .map(|(i, _)| Coords {
                x: i as i64 % self.width,
                y: i as i64 / self.width,
            })
    }
}

fn forward(p: Coords, d: Direction) -> Coords {
    p + direction_vector(d)
}

struct State {
    pos: Coords,
    dir: Direction,
    score: i64,
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let p0 = State {
        pos: start,
        dir: Direction::East,
        score: 0,
    };
    let mut hits = HashMap::new();
    let mut todo = vec![p0];
    while let Some(s) = todo.pop() {
        if grid.get(s.pos) == '#' {
            continue;
        };

        if let Some(score2) = hits.get(&s.pos) {
            if *score2 <= s.score {
                continue;
            }
            *hits.get_mut(&s.pos).unwrap() = s.score;
        } else {
            hits.insert(s.pos, s.score);
        }

        if grid.get(s.pos) == 'E' {
            continue;
        };

        todo.push(State {
            pos: forward(s.pos, turn_left(s.dir)),
            dir: turn_left(s.dir),
            score: s.score + 1001,
        });
        todo.push(State {
            pos: forward(s.pos, turn_right(s.dir)),
            dir: turn_right(s.dir),
            score: s.score + 1001,
        });
        todo.push(State {
            pos: forward(s.pos, s.dir),
            dir: (s.dir),
            score: s.score + 1,
        });
    }

    let r1 = hits.get(&end).unwrap();
    println!("{}", r1);
}
