use std::{collections::HashSet, fmt, io};

use itertools::iproduct;

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

    pub fn get_neightbours(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *x < self.width && *y >= 0 && *y < self.height)
            .collect()
    }
}

fn find_trailheads(grid: &Grid<u8>) -> Vec<(i64, i64)> {
    iproduct!(0..grid.width, 0..grid.height)
        .filter(|&(x, y)| grid.get(x, y) == 0)
        .collect()
}

fn score_trailhead(grid: &Grid<u8>, trailhead: (i64, i64)) -> usize {
    let mut peaks = HashSet::new();
    let mut todo = vec![(trailhead, 0)];
    while let Some(((x, y), height)) = todo.pop() {
        if height == 9 {
            peaks.insert((x, y));
            continue;
        }
        let ns = grid.get_neightbours(x, y);
        ns.into_iter()
            .filter(|c| grid.get(c.0, c.1) == height + 1)
            .for_each(|c| {
                todo.push((c, height + 1));
            });
    }
    peaks.len()
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let trailheads = find_trailheads(&grid);
    let r1 = trailheads
        .into_iter()
        .map(|t| score_trailhead(&grid, t))
        .sum::<usize>();
    println!("{}", r1);
}
