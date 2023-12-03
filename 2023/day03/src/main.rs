use std::{collections::BTreeMap, io};

use itertools::Itertools;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn has_adjacent_symbol(grid: &Vec<Vec<char>>, x: usize, y: usize, l: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    for i in (x as i32 - 1).max(0)..(x + l + 1).min(width) as i32 {
        if (y as i32 - 1) >= 0 {
            let c = grid[y - 1][i as usize];
            if is_symbol(c) {
                return true;
            }
        }
        if y + 1 < height {
            let c = grid[y + 1][i as usize];
            if is_symbol(c) {
                return true;
            }
        }
    }
    if (x as i32 - 1) >= 0 {
        let c = grid[y][x - 1];
        if is_symbol(c) {
            return true;
        }
    }
    if x + l < width {
        let c = grid[y][x + l];
        if is_symbol(c) {
            return true;
        }
    }
    false
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn get_gears(grid: &Vec<Vec<char>>, x: usize, y: usize, l: usize) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut gears = vec![];
    for i in (x as i32 - 1).max(0)..(x + l + 1).min(width) as i32 {
        if (y as i32 - 1) >= 0 {
            let c = grid[y - 1][i as usize];
            if is_gear(c) {
                gears.push((i as usize, y - 1));
            }
        }
        if y + 1 < height {
            let c = grid[y + 1][i as usize];
            if is_gear(c) {
                gears.push((i as usize, y + 1));
            }
        }
    }
    if (x as i32 - 1) >= 0 {
        let c = grid[y][x - 1];
        if is_gear(c) {
            gears.push((x - 1, y));
        }
    }
    if x + l < width {
        let c = grid[y][x + l];
        if is_gear(c) {
            gears.push((x + l, y));
        }
    }
    gears
}

#[derive(Debug)]
struct Number {
    pub value: u32,
    pub x: usize,
    pub y: usize,
    pub l: usize,
}

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let part_numbers = (0..height)
        .flat_map(|y| {
            let mut numbers = vec![];
            let mut x = 0;
            while x < width {
                let mut n = 0;
                let mut l = 0usize;
                while x + l < width && grid[y][x + l].is_ascii_digit() {
                    n = n * 10 + grid[y][x + l].to_digit(10).unwrap();
                    l += 1;
                }
                if n > 0 {
                    numbers.push(Number { value: n, x, y, l });
                }
                x += l + 1;
            }
            numbers
        })
        .filter_map(|n| {
            if has_adjacent_symbol(&grid, n.x, n.y, n.l) {
                Some(n)
            } else {
                None
            }
        })
        .collect_vec();
    let r1 = part_numbers.iter().map(|n| n.value).sum::<u32>();
    println!("{}", r1);

    let r2 = part_numbers
        .iter()
        .flat_map(|n| {
            get_gears(&grid, n.x, n.y, n.l)
                .into_iter()
                .map(|g| (n.value, g))
        })
        .fold(BTreeMap::new(), |mut acc, (n, g)| {
            acc.entry(g).or_insert(vec![]).push(n);
            acc
        })
        .into_iter()
        .filter(|(_k, v)| v.len() == 2)
        .map(|(_k, v)| v[0] as u64 * v[1] as u64)
        .sum::<u64>();
    println!("{:?}", r2);
}
