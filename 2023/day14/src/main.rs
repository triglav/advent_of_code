use std::{collections::HashMap, fmt::Display, io};

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

    fn slide_single_south(&mut self, x: usize, y: usize) {
        assert!(self.get(x, y) == Tile::Empty);
        let y2 = (0..y).rev().position(|y| self.get(x, y) != Tile::Empty);
        if let Some(y2) = y2 {
            let y2 = y - y2 - 1;
            let t = self.get(x, y2);
            if t == Tile::RoundRock {
                self.grid[y][x] = t;
                self.grid[y2][x] = Tile::Empty;
            }
        }
    }

    fn slide_single_east(&mut self, x: usize, y: usize) {
        assert!(self.get(x, y) == Tile::Empty);
        let x2 = (0..x).rev().position(|x| self.get(x, y) != Tile::Empty);
        if let Some(x2) = x2 {
            let x2 = x - x2 - 1;
            let t = self.get(x2, y);
            if t == Tile::RoundRock {
                self.grid[y][x] = t;
                self.grid[y][x2] = Tile::Empty;
            }
        }
    }

    fn slide_single_west(&mut self, x: usize, y: usize) {
        assert!(self.get(x, y) == Tile::Empty);
        let x2 = (x + 1..self.width).position(|x| self.get(x, y) != Tile::Empty);
        if let Some(x2) = x2 {
            let x2 = x2 + x + 1;
            let t = self.get(x2, y);
            if t == Tile::RoundRock {
                self.grid[y][x] = t;
                self.grid[y][x2] = Tile::Empty;
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

    fn tilt_south(&mut self) {
        (0..self.width).for_each(|x| {
            (0..self.height).rev().for_each(|y| {
                if self.get(x, y) == Tile::Empty {
                    self.slide_single_south(x, y);
                }
            });
        });
    }

    fn tilt_east(&mut self) {
        (0..self.height).for_each(|y| {
            (0..self.width).rev().for_each(|x| {
                if self.get(x, y) == Tile::Empty {
                    self.slide_single_east(x, y);
                }
            });
        });
    }

    fn tilt_west(&mut self) {
        (0..self.height).for_each(|y| {
            (0..self.width).for_each(|x| {
                if self.get(x, y) == Tile::Empty {
                    self.slide_single_west(x, y);
                }
            });
        });
    }

    fn spin_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
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

    fn get_key(&self) -> String {
        self.grid.iter().flatten().map(|t| t.to_string()).collect()
    }
}

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().map(parse_tile).collect::<Vec<Tile>>())
        .collect::<Vec<Vec<Tile>>>();
    let mut p = Platform::from(grid);
    p.tilt_north();
    let r1 = p.calculate_load();
    println!("{}", r1);

    p.tilt_west();
    p.tilt_south();
    p.tilt_east();

    let mut seen = HashMap::new();
    seen.insert(p.get_key(), 0);

    let total = 1_000_000_000;
    for i in 2..=total {
        p.spin_cycle();
        let key = p.get_key();
        if let Some(i0) = seen.get(&key) {
            let d = i - i0;
            for _ in 0..d {
                p.spin_cycle();
            }
            assert_eq!(p.get_key(), key);

            let t2 = total - i0;
            let d2 = t2 % d;
            for _ in 0..d2 {
                p.spin_cycle();
            }
            break;
        }
        seen.insert(key, i);
    }
    let r2 = p.calculate_load();
    println!("{}", r2);
}
