use std::{fmt, io};

use itertools::{iproduct, Itertools};

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

    pub fn get_neightbours(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *x < self.width && *y >= 0 && *y < self.height)
            .collect()
    }
}

fn trace_region(grid: &mut Grid<char>, x: i64, y: i64) -> Option<(u64, u64)> {
    let t0 = grid.get(x, y);
    if t0 == '.' || t0.is_ascii_lowercase() {
        return None;
    }
    let tt = t0.to_ascii_lowercase();
    let mut area = 0;
    let mut perimeter = 0;
    let mut todo = vec![(x, y)];
    while let Some((x, y)) = todo.pop() {
        let t = grid.get(x, y);
        if t == tt {
            continue;
        }
        if t != t0 {
            panic!("trace_region: t({}) != t0({}) ({},{})", t, t0, x, y);
        }
        *grid.get_mut(x, y) = tt;
        area += 1;
        let ns = grid.get_neightbours(x, y);
        let ns_perimeter = ns
            .iter()
            .filter(|(x, y)| grid.get(*x, *y) == t0 || grid.get(*x, *y) == tt)
            .collect_vec();
        perimeter += 4 - ns_perimeter.len() as u64;
        todo.extend(ns.into_iter().filter(|(x, y)| grid.get(*x, *y) == t0));
    }
    Some((area, perimeter))
}

fn solve1(grid: &Grid<char>) -> u64 {
    let mut grid = grid.clone();
    iproduct!(0..grid.height, 0..grid.width,)
        .filter_map(|(y, x)| trace_region(&mut grid, x, y))
        .map(|(a, p)| a * p)
        .sum::<u64>()
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let r1 = solve1(&grid);
    println!("{}", r1);
}
