use std::{
    collections::{HashMap, VecDeque},
    io,
};

struct Grid<T> {
    pub width: i32,
    pub height: i32,
    tiles: Vec<T>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn from(grid: Vec<Vec<T>>) -> Grid<T> {
        let width = grid[0].len() as i32;
        let height = grid.len() as i32;
        let tiles = grid.into_iter().flatten().collect::<Vec<_>>();
        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles[(y * self.width + x) as usize]
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_direction_coord<T>(grid: &Grid<T>, x: i32, y: i32, dir: Direction) -> Option<(i32, i32)> {
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

const MAX_STRAIGHT: i32 = 3;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Crucible {
    x: i32,
    y: i32,
    dir: Direction,
    remaining_straight: i32,
    heat_loss: u32,
}

impl Crucible {
    fn advance(
        &self,
        grid: &Grid<u32>,
        dir: Direction,
        remaining_straight: i32,
    ) -> Option<Crucible> {
        get_direction_coord(grid, self.x, self.y, dir).map(|(x, y)| Crucible {
            x,
            y,
            dir,
            remaining_straight,
            heat_loss: self.heat_loss + grid.get(x, y),
        })
    }

    pub fn step(&self, grid: &Grid<u32>) -> Vec<Crucible> {
        let mut r = vec![];
        if self.remaining_straight > 0 {
            if let Some(c) = self.advance(grid, self.dir, self.remaining_straight - 1) {
                r.push(c);
            }
        }
        if let Some(c) = self.advance(grid, self.dir.right(), MAX_STRAIGHT - 1) {
            r.push(c);
        }
        if let Some(c) = self.advance(grid, self.dir.left(), MAX_STRAIGHT - 1) {
            r.push(c);
        }
        r
    }
}

fn advance(crucible: Crucible, grid: &Grid<u32>) -> u32 {
    let mut todo = VecDeque::from([crucible]);

    // coords, dir, remaining straight -> heat
    let visited = &mut HashMap::<(i32, i32, Direction, i32), u32>::new();
    while let Some(crucible) = todo.pop_front() {
        if let Some(h2) = visited.get(&(
            crucible.x,
            crucible.y,
            crucible.dir,
            crucible.remaining_straight,
        )) {
            if crucible.heat_loss >= *h2 {
                continue;
            }
        }
        visited.insert(
            (
                crucible.x,
                crucible.y,
                crucible.dir,
                crucible.remaining_straight,
            ),
            crucible.heat_loss,
        );
        todo.extend(crucible.step(grid));
    }
    visited
        .iter()
        .filter_map(|((x, y, _dir, _rs), h)| {
            if (*x, *y) == (grid.width - 1, grid.height - 1) {
                Some(*h)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let r1 = advance(
        Crucible {
            x: 0,
            y: 0,
            dir: Direction::Right,
            remaining_straight: MAX_STRAIGHT,
            heat_loss: 0,
        },
        &grid,
    );
    println!("{}", r1);
}
