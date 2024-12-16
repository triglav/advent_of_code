use std::ops::{Add, Div, Mul, Sub};
use std::{fmt, io};

use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_direction(c: char) -> Direction {
    match c {
        '<' => Direction::Left,
        '>' => Direction::Right,
        '^' => Direction::Up,
        'v' => Direction::Down,
        _ => panic!("Unexpected  direction character"),
    }
}

fn direction_vector(d: Direction) -> Coords {
    match d {
        Direction::Up => Coords { x: 0, y: -1 },
        Direction::Down => Coords { x: 0, y: 1 },
        Direction::Left => Coords { x: -1, y: 0 },
        Direction::Right => Coords { x: 1, y: 0 },
    }
}

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

    pub fn get_mut(&mut self, p: Coords) -> &mut T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles
            .get_mut((p.y * self.width + p.x) as usize)
            .unwrap()
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

fn gps_coords(c: Coords) -> i64 {
    100 * c.y + c.x
}

fn push(grid: &mut Grid<char>, p: Coords, d: Direction) -> Option<Coords> {
    let v = direction_vector(d);
    let dest = p.add(v);
    let dest_tile = grid.get(dest);
    match dest_tile {
        '#' => None,
        '.' => {
            let t = grid.get(p);
            *grid.get_mut(dest) = t;
            *grid.get_mut(p) = '.';
            Some(dest)
        }
        'O' => match push(grid, dest, d) {
            Some(_) => push(grid, p, d),
            None => None,
        },
        _ => panic!("Unexpected tile"),
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect_vec();
    let (grid_lines, moves_lines): (Vec<_>, Vec<_>) =
        lines.into_iter().partition(|l| l.starts_with('#'));

    let mut grid = Grid::from(
        grid_lines
            .into_iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let moves = moves_lines
        .into_iter()
        .join("")
        .chars()
        .map(parse_direction)
        .collect_vec();

    let mut robot_pos = grid.find('@').unwrap();
    *grid.get_mut(robot_pos) = '.';

    for m in moves {
        if let Some(new_pos) = push(&mut grid, robot_pos, m) {
            robot_pos = new_pos;
        }
    }

    let r1 = iproduct!(0..grid.height, 0..grid.width)
        .map(|(y, x)| Coords { x, y })
        .filter(move |&c| grid.get(c) == 'O')
        .map(gps_coords)
        .sum::<i64>();
    println!("{}", r1);
}
