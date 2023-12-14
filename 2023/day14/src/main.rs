use std::{fmt::Display, io};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    RoundRock,
    CubeRock,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => '.',
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
        };
        write!(f, "{}", c)
    }
}

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Empty,
        'O' => Tile::RoundRock,
        '#' => Tile::CubeRock,
        _ => panic!("Invalid tile: {}", c),
    }
}

#[derive(Clone)]
struct Platform {
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
}

impl Platform {
    fn from(grid: Vec<Vec<Tile>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Platform {
            width,
            height,
            grid,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.grid[y][x]
    }

    fn slide_single_north(&mut self, x: usize, y: usize) {
        assert!(self.get(x, y) == Tile::Empty);
        let y2 = (y + 1..self.height).position(|y| self.get(x, y) != Tile::Empty);
        if let Some(y2) = y2 {
            let y2 = y2 + y + 1;
            let t = self.get(x, y2);
            if t == Tile::RoundRock {
                self.grid[y][x] = t;
                self.grid[y2][x] = Tile::Empty;
            }
        }
    }

    fn tilt_north(&mut self) {
        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                if self.get(x, y) == Tile::Empty {
                    self.slide_single_north(x, y);
                }
            });
        });
    }

    fn calculate_load(&self) -> u32 {
        (0..self.width)
            .flat_map(|x| {
                (0..self.height)
                    .map(|y| (y, self.get(x, y)))
                    .collect::<Vec<_>>()
            })
            .filter(|(_, t)| *t == Tile::RoundRock)
            .map(|(y, _)| (self.height - y) as u32)
            .sum::<u32>()
    }
}

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().map(parse_tile).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();
    let mut p = Platform::from(grid);
    p.tilt_north();
    let r2 = p.calculate_load();
    println!("{}", r2);
}
