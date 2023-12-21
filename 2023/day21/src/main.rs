use std::{collections::VecDeque, io};

use itertools::iproduct;

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

    pub fn get_neightbours(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *x < self.width && *y >= 0 && *y < self.height)
            .collect()
    }
}

fn trace(grid: &Grid<char>, start: (i32, i32)) -> Grid<Option<u32>> {
    let mut hit_grid = Grid::new(grid.width, grid.height, None);
    let mut todo = VecDeque::from([((start.0, start.0), 0u32)]);
    while let Some(((x, y), steps)) = todo.pop_front() {
        let s = hit_grid.get_mut(x, y);
        if s.is_some() {
            continue;
        }
        *s = Some(steps);
        grid.get_neightbours(x, y).into_iter().for_each(|(x, y)| {
            if grid.get(x, y) != '#' {
                todo.push_back(((x, y), steps + 1));
            }
        });
    }
    hit_grid
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let grid = Grid::from(lines);
    let start = iproduct!(0..grid.width, 0..grid.height)
        .find(|(x, y)| grid.get(*x, *y) == 'S')
        .unwrap();

    let hit_grid = trace(&grid, start);
    const STEPS: u32 = 64;
    let r1 = iproduct!(0..hit_grid.width, 0..hit_grid.height).filter_map(|(x, y)| {
        if let Some(s) = hit_grid.get(x, y) {
            (s == STEPS || (s < STEPS && s % 2 == 0))
                .then_some(s)
                .or(None)
        } else {
            None
        }
    });
    println!("{}", r1.count());
}
