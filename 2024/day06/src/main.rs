use std::{collections::HashSet, fmt, io};

use itertools::iproduct;

#[derive(Clone)]
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

    pub fn get_mut(&mut self, x: i64, y: i64) -> &mut T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles.get_mut((y * self.width + x) as usize).unwrap()
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

enum AdvanceResult {
    Ok(Guard),
    OutOfBounds,
    LoopDetected,
}

impl Guard {
    pub fn new(x: i64, y: i64, dir: Direction) -> Guard {
        Guard { x, y, dir }
    }

    pub fn advance(
        &self,
        grid: &Grid<char>,
        visited: &mut HashSet<(i64, i64)>,
        loop_guard: &mut HashSet<(i64, i64, Direction)>,
    ) -> AdvanceResult {
        loop_guard.insert((self.x, self.y, self.dir));
        if let Some((x, y)) = get_direction_coord(grid, self.x, self.y, self.dir) {
            let t = grid.get(x, y);
            if t == '#' {
                AdvanceResult::Ok(Guard {
                    x: self.x,
                    y: self.y,
                    dir: self.dir.right(),
                })
            } else {
                if loop_guard.contains(&(x, y, self.dir)) {
                    return AdvanceResult::LoopDetected;
                }
                visited.insert((x, y));
                loop_guard.insert((x, y, self.dir));
                AdvanceResult::Ok(Guard {
                    x,
                    y,
                    dir: self.dir,
                })
            }
        } else {
            AdvanceResult::OutOfBounds
        }
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
    let guard0 = Guard::new(x, y, Direction::Up);

    let mut guard = guard0.clone();
    let mut visited = HashSet::new();
    let mut loop_guard = HashSet::new();
    let mut todo = HashSet::new();
    loop {
        guard = match guard.advance(&grid, &mut visited, &mut loop_guard) {
            AdvanceResult::Ok(guard) => guard,
            AdvanceResult::OutOfBounds => break,
            AdvanceResult::LoopDetected => panic!(),
        };
        todo.insert((guard.x, guard.y));
    }
    let r1 = visited.len();
    println!("{}", r1);

    let r2 = todo
        .into_iter()
        .map(|c| {
            let mut grid2 = grid.clone();
            if grid2.get(c.0, c.1) == '^' {
                return false;
            }
            *grid2.get_mut(c.0, c.1) = '#';
            let mut guard2 = guard0.clone();
            let mut visited2 = HashSet::new();
            let mut loop_guard2 = HashSet::new();
            loop {
                guard2 = match guard2.advance(&grid2, &mut visited2, &mut loop_guard2) {
                    AdvanceResult::Ok(guard) => guard,
                    AdvanceResult::OutOfBounds => break,
                    AdvanceResult::LoopDetected => return true,
                };
            }
            false
        })
        .filter(|&b| b)
        .count();
    println!("{}", r2);
}
