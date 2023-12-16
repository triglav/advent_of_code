use std::{
    collections::{HashSet, VecDeque},
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
    pub fn new(width: i32, height: i32, default: T) -> Grid<T> {
        let tiles = vec![default; (width * height) as usize];
        Grid {
            width,
            height,
            tiles,
        }
    }

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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Laser {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Laser {
    fn advance(&self, grid: &Grid<char>) -> Vec<Laser> {
        let (x, y) = match self.dir {
            Direction::Up => (self.x, (self.y - 1).max(0)),
            Direction::Down => (self.x, (self.y + 1).min(grid.height - 1)),
            Direction::Left => ((self.x - 1).max(0), self.y),
            Direction::Right => ((self.x + 1).min(grid.width - 1), self.y),
        };
        if x == self.x && y == self.y {
            // No movement
            return vec![];
        }
        let d = match grid.get(x, y) {
            '.' => vec![self.dir],
            '/' => match self.dir {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            },
            '\\' => match self.dir {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
            },
            '|' => match self.dir {
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
                _ => vec![self.dir],
            },
            '-' => match self.dir {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                _ => vec![self.dir],
            },
            _ => panic!("Unexpected tile"),
        };
        d.into_iter()
            .map(|dir| Laser { x, y, dir })
            .collect::<Vec<_>>()
    }
}

fn energise(grid: &Grid<char>, start: Laser) -> usize {
    let mut visited = HashSet::new();
    let mut energised = Grid::new(grid.width, grid.height, false);
    let mut todo = VecDeque::from([start]);
    while let Some(laser) = todo.pop_front() {
        if visited.contains(&laser) {
            continue;
        }
        visited.insert(laser);
        *energised.get_mut(laser.x, laser.y) = true;

        todo.extend(laser.advance(grid));
    }
    energised.tiles.iter().filter(|&&e| e).count()
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let r1 = energise(
        &grid,
        Laser {
            x: 0,
            y: 0,
            dir: Direction::Right,
        },
    );
    println!("{}", r1);

    let r2 = (0..grid.width)
        .flat_map(|x| {
            vec![
                Laser {
                    x,
                    y: 0,
                    dir: Direction::Down,
                },
                Laser {
                    x,
                    y: grid.height - 1,
                    dir: Direction::Up,
                },
            ]
        })
        .chain((0..grid.height).flat_map(|y| {
            vec![
                Laser {
                    x: 0,
                    y,
                    dir: Direction::Right,
                },
                Laser {
                    x: grid.width - 1,
                    y,
                    dir: Direction::Left,
                },
            ]
        }))
        .map(|laser| energise(&grid, laser))
        .max()
        .unwrap();
    println!("{}", r2);
}
