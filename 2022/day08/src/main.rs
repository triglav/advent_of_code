use std::{collections::HashSet, io};

type Grid = Vec<Vec<i8>>;

#[derive(PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

fn read_grid() -> Grid {
    let mut grid = Grid::new();
    for line in io::stdin().lines().map(|l| l.unwrap()) {
        let mut v = Vec::new();
        for c in line.chars() {
            let x = c.to_digit(10).unwrap();
            let d = i8::try_from(x).unwrap();
            v.push(d);
        }
        grid.push(v);
    }
    grid
}

fn filter_visible<'a, I>(iter: I) -> Vec<usize>
where
    I: Iterator<Item = &'a i8>,
{
    let mut t0 = -1_i8;
    let mut v = Vec::<usize>::new();
    for (idx, t) in iter.enumerate() {
        if *t > t0 {
            v.push(idx);
            t0 = *t;
        }
    }
    v
}

fn filter_visible_from_left(grid: &Grid) -> HashSet<Coord> {
    let mut m = HashSet::<Coord>::new();
    for (y, row) in grid.iter().enumerate() {
        for x in filter_visible(row.iter()) {
            m.insert(Coord { x, y });
        }
    }
    m
}

fn filter_visible_from_right(grid: &Grid) -> HashSet<Coord> {
    let width = grid.first().unwrap().len();
    let mut m = HashSet::<Coord>::new();
    for (y, row) in grid.iter().enumerate() {
        for x in filter_visible(row.iter().rev()) {
            m.insert(Coord {
                x: width - 1 - x,
                y,
            });
        }
    }
    m
}

fn filter_visible_from_top(grid: &Grid) -> HashSet<Coord> {
    let height = grid.len();
    let width = grid.first().unwrap().len();

    let mut m = HashSet::<Coord>::new();
    for x in 0..width {
        let iter = (0..height).map(|y| &grid[y][x]);
        for y in filter_visible(iter) {
            m.insert(Coord { x, y });
        }
    }
    m
}

fn filter_visible_from_bottom(grid: &Grid) -> HashSet<Coord> {
    let height = grid.len();
    let width = grid.first().unwrap().len();

    let mut m = HashSet::<Coord>::new();
    for x in 0..width {
        let iter = (0..height).rev().map(|y| &grid[y][x]);
        for y in filter_visible(iter) {
            m.insert(Coord {
                x,
                y: height - 1 - y,
            });
        }
    }
    m
}

fn main() {
    let grid = read_grid();

    let from_left = filter_visible_from_left(&grid);
    let from_right = filter_visible_from_right(&grid);
    let from_top = filter_visible_from_top(&grid);
    let from_bottom = filter_visible_from_bottom(&grid);

    let mut visible = HashSet::<Coord>::new();
    visible.extend(from_left);
    visible.extend(from_right);
    visible.extend(from_top);
    visible.extend(from_bottom);

    let r1 = visible.len();
    println!("{}", r1);
}
