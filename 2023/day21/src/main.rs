use core::fmt;
use std::{collections::VecDeque, io};

use itertools::iproduct;

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
    pub fn new(width: i64, height: i64, default: T) -> Grid<T> {
        let tiles = vec![default; (width * height) as usize];
        Grid {
            width,
            height,
            tiles,
        }
    }

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

fn trace(grid: &Grid<char>, start: (i64, i64), start_steps: u64) -> Grid<Option<u64>> {
    let mut hit_grid = Grid::new(grid.width, grid.height, None);
    let mut todo = VecDeque::from([((start.0, start.1), start_steps)]);
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

fn count_steps(hit_grid: &Grid<Option<u64>>, steps: u64) -> u64 {
    if steps == 0 {
        return 0;
    }
    let z = steps % 2;
    iproduct!(0..hit_grid.width, 0..hit_grid.height)
        .filter_map(|(x, y)| {
            if let Some(s) = hit_grid.get(x, y) {
                (s == steps || (s < steps && s % 2 == z))
                    .then_some(s)
                    .or(None)
            } else {
                None
            }
        })
        .count() as u64
}

//    odo
//   oidio
//  oi.d.io
// oi..d..io
// ddddsdddd
// oi..d..io
//  oi.d.io
//   oidio
//    odo

fn solve_corner(grid: &Grid<char>, start: (i64, i64), steps: u64) -> u64 {
    let steps_initial = grid.width as u64 + 1;
    let steps_corner = steps - steps_initial;

    let grids = steps_corner / grid.width as u64;
    let steps_outer_remaining = steps_corner % grid.width as u64;
    let steps_inner_remaining = steps_corner % grid.width as u64 + grid.width as u64;
    assert!(steps_outer_remaining > 0);

    let grids_inner = grids;
    let grids_outer = grids + 1;

    let grids_corner_odd = (1..grids).filter(|i| i % 2 == 1).sum::<u64>();
    let grids_corner_even = (1..grids).filter(|i| i % 2 == 0).sum::<u64>();

    let hit_grid = trace(grid, start, 0);

    let steps_odd = count_steps(&hit_grid, steps_corner);
    let steps_even = count_steps(&hit_grid, steps_corner + 1);
    let steps_inner = count_steps(&hit_grid, steps_inner_remaining);
    let steps_outer = count_steps(&hit_grid, steps_outer_remaining);
    assert_eq!(steps_inner_remaining % 2, 1);
    assert_eq!(steps_outer_remaining % 2, 0);

    let before_last = count_steps(&hit_grid, steps_inner_remaining + grid.width as u64);
    assert_eq!(before_last, steps_even);

    steps_odd * grids_corner_odd
        + steps_even * grids_corner_even
        + steps_inner * grids_inner
        + steps_outer * grids_outer
}

fn solve_direct(grid: &Grid<char>, start: (i64, i64), steps: u64) -> u64 {
    let steps_initial = grid.width as u64 / 2 + 1;
    let steps_direct = steps - steps_initial;

    let grids = steps_direct / grid.width as u64;
    let steps_remaining = steps_direct % grid.width as u64;
    assert!(steps_remaining > 0);

    let grids_odd = grids / 2 + grids % 2;
    let grids_even = grids / 2;

    let hit_grid = trace(grid, start, 0);

    let steps_odd = count_steps(&hit_grid, steps_direct);
    let steps_even = count_steps(&hit_grid, steps_direct + 1);
    let steps_last = count_steps(&hit_grid, steps_remaining);

    let before_last = count_steps(&hit_grid, steps_remaining + grid.width as u64);
    assert_eq!(before_last, steps_odd);

    steps_odd * grids_odd + steps_even * grids_even + steps_last
}

fn solve2(grid: &Grid<char>, hit_grid: &Grid<Option<u64>>, steps: u64) -> u64 {
    assert_eq!(grid.width, grid.height);
    let z = grid.width / 2;

    let base = count_steps(hit_grid, steps);

    let right = solve_direct(grid, (0, z), steps);
    let left = solve_direct(grid, (grid.width - 1, z), steps);
    let up = solve_direct(grid, (z, 0), steps);
    let down = solve_direct(grid, (z, grid.height - 1), steps);

    let up_right = solve_corner(grid, (0, grid.height - 1), steps);
    let up_left = solve_corner(grid, (grid.width - 1, grid.height - 1), steps);
    let down_right = solve_corner(grid, (0, 0), steps);
    let down_left = solve_corner(grid, (grid.width - 1, 0), steps);

    base + right + left + up + down + up_right + up_left + down_right + down_left
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
    let hit_grid = trace(&grid, start, 0);
    let r1 = count_steps(&hit_grid, 64);
    println!("{}", r1);

    const STEPS2: u64 = 26501365;
    let r2 = solve2(&grid, &hit_grid, STEPS2);
    assert_eq!(r2, 609012263058042);
    println!("{}", r2);
}
