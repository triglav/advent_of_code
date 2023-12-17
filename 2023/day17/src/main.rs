use std::{collections::VecDeque, io};

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

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles.get_mut((y * self.width + x) as usize).unwrap()
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Crucible {
    x: i32,
    y: i32,
    dir: Direction,
    heat_loss: u32,
}

impl Crucible {
    pub fn new(dir: Direction) -> Crucible {
        Crucible {
            x: 0,
            y: 0,
            dir,
            heat_loss: 0,
        }
    }

    fn advance(&self, grid: &Grid<u32>, dir: Direction) -> Option<Crucible> {
        get_direction_coord(grid, self.x, self.y, dir).map(|(x, y)| Crucible {
            x,
            y,
            dir,
            heat_loss: self.heat_loss + grid.get(x, y),
        })
    }

    pub fn step(&self, grid: &Grid<u32>, min_straight: i32, max_straight: i32) -> Vec<Crucible> {
        let mut r = vec![];
        let mut c = Some(*self);
        for i in 1..=max_straight {
            if let Some(c2) = c {
                c = c2.advance(grid, c2.dir);
                if let Some(c) = c {
                    if i >= min_straight {
                        r.push(c);
                    }
                }
            }
        }
        r.into_iter()
            .flat_map(|c| {
                vec![
                    Crucible {
                        x: c.x,
                        y: c.y,
                        dir: c.dir.left(),
                        heat_loss: c.heat_loss,
                    },
                    Crucible {
                        x: c.x,
                        y: c.y,
                        dir: c.dir.right(),
                        heat_loss: c.heat_loss,
                    },
                ]
            })
            .collect::<Vec<_>>()
    }
}

fn advance(start: Vec<Crucible>, grid: &Grid<u32>, min_straight: i32, max_straight: i32) -> u32 {
    let mut todo = VecDeque::from(start);
    let mut visited = Grid {
        width: grid.width,
        height: grid.height,
        tiles: vec![[None, None, None, None]; (grid.width * grid.height) as usize],
    };
    while let Some(crucible) = todo.pop_front() {
        let h2 = &mut visited.get_mut(crucible.x, crucible.y)[crucible.dir as usize];
        if let Some(h2) = h2 {
            if crucible.heat_loss >= *h2 {
                continue;
            }
        }
        *h2 = Some(crucible.heat_loss);
        todo.extend(crucible.step(grid, min_straight, max_straight));
    }
    visited
        .get(grid.width - 1, grid.height - 1)
        .into_iter()
        .flatten()
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
        vec![
            Crucible::new(Direction::Right),
            Crucible::new(Direction::Down),
        ],
        &grid,
        0,
        3,
    );
    println!("{}", r1);

    let r2 = advance(
        vec![
            Crucible::new(Direction::Right),
            Crucible::new(Direction::Down),
        ],
        &grid,
        4,
        10,
    );
    println!("{}", r2);
}
