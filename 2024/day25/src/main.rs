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
}

fn is_lock(g: &Grid<char>) -> bool {
    (0..g.width).all(|x| g.get(x, 0) == '#')
}

fn to_heights(g: &Grid<char>) -> Vec<usize> {
    (0..g.width)
        .map(|x| (0..g.height).filter(|y| g.get(x, *y) == '#').count() - 1)
        .collect()
}

fn overlaps(lock: &[usize], key: &[usize], max_height: usize) -> bool {
    lock.iter()
        .zip(key.iter())
        .any(|(l, k)| *l + *k > max_height)
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let (locks, keys): (Vec<_>, Vec<_>) = lines
        .iter()
        .chunk_by(|l| l.is_empty())
        .into_iter()
        .map(|(_, c)| c.collect_vec())
        .filter(|c| c.len() > 1)
        .map(|c| {
            Grid::from(
                c.into_iter()
                    .map(|l| l.chars().collect::<Vec<_>>())
                    .collect(),
            )
        })
        .partition(is_lock);
    let max_height = locks[0].height as usize - 2;
    let locks_heights = locks.iter().map(to_heights).collect_vec();
    let keys_heights = keys.iter().map(to_heights).collect_vec();
    let r1 = locks_heights
        .iter()
        .cartesian_product(keys_heights.iter())
        .map(|(l, k)| overlaps(l, k, max_height))
        .filter(|overlaps| !overlaps)
        .count();
    println!("{}", r1);
}
