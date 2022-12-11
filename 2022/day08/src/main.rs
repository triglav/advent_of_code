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

fn count_visibe_trees<I>(t: i8, iter: I) -> u64
where
    I: Iterator<Item = i8>,
{
    let mut r = 0_u64;
    for t2 in iter {
        r += 1;
        if t2 >= t {
            break;
        }
    }
    r
}

fn get_scenic_score(c: Coord, grid: &Grid, width: usize, height: usize) -> u64 {
    let t = grid[c.y][c.x];
    let top = (0..c.y).rev().map(|y| grid[y][c.x]);
    let bottom = ((c.y + 1)..height).map(|y| grid[y][c.x]);
    let left = (0..c.x).rev().map(|x| grid[c.y][x]);
    let right = ((c.x + 1)..width).map(|x| grid[c.y][x]);

    count_visibe_trees(t, top)
        * count_visibe_trees(t, bottom)
        * count_visibe_trees(t, left)
        * count_visibe_trees(t, right)
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

    let height = grid.len();
    let width = grid.first().unwrap().len();
    let r2 = (0..height)
        .flat_map(move |y| (0..width).map(move |x| Coord { x, y }))
        .map(|c| get_scenic_score(c, &grid, width, height))
        .max()
        .unwrap();
    println!("{}", r2);
}
