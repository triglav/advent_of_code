use std::{collections::HashSet, fmt, io};

use itertools::iproduct;

struct Grid<T> {
    pub width: i64,
    pub height: i64,
    tiles: Vec<T>,
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

    pub fn get(&self, x: i64, y: i64) -> T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn find(&self, t: T) -> Option<(i64, i64)> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, &x)| x == t)
            .map(|(i, _)| (i as i64 % self.width, i as i64 / self.width))
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down,
    Left,
    Right,
}

impl Direction {
    fn right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_direction_coord<T>(grid: &Grid<T>, x: i64, y: i64, dir: Direction) -> Option<(i64, i64)> {
    assert!(x >= 0 && x < grid.width);
    assert!(y >= 0 && y < grid.height);
    let (x, y) = match dir {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    };
    if x < 0 || x >= grid.width {
        return None;
    }
    if y < 0 || y >= grid.height {
        return None;
    }
    Some((x, y))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    x: i64,
    y: i64,
    dir: Direction,
}

impl Guard {
    pub fn new(x: i64, y: i64, dir: Direction) -> Guard {
        Guard { x, y, dir }
    }

    pub fn advance(&self, grid: &Grid<char>, visited: &mut HashSet<(i64, i64)>) -> Option<Guard> {
        get_direction_coord(grid, self.x, self.y, self.dir).map(move |(x, y)| {
            let t = grid.get(x, y);
            if t == '#' {
                Guard {
                    x: self.x,
                    y: self.y,
                    dir: self.dir.right(),
                }
            } else {
                visited.insert((x, y));
                Guard {
                    x,
                    y,
                    dir: self.dir,
                }
            }
        })
    }
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let (x, y) = grid.find('^').unwrap();
    let mut guard = Guard::new(x, y, Direction::Up);
    let mut visited = HashSet::new();
    loop {
        guard = match guard.advance(&grid, &mut visited) {
            Some(guard) => guard,
            None => break,
        };
    }
    let r1 = visited.len();
    println!("{}", r1);
}
