use std::{collections::HashMap, fmt, io};

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

    pub fn is_valid(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn get_neightbours(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|&(x, y)| self.is_valid(x, y))
            .collect()
    }
}

fn trace_region(grid: &mut Grid<char>, x: i64, y: i64) -> Option<(u64, u64, u64)> {
    let t0 = grid.get(x, y);
    if t0 == '.' || t0.is_ascii_lowercase() {
        return None;
    }
    let tt = t0.to_ascii_lowercase();
    let mut area = 0;
    let mut perimeter = 0;
    let mut todo = vec![(x, y)];
    let mut perimeter_edges = HashMap::new();
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

        let edges = vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter(|(dx, dy)| {
                let x2 = x + dx;
                let y2 = y + dy;
                if !grid.is_valid(x2, y2) {
                    return true;
                }
                let tp = grid.get(x2, y2);
                tp != t0 && tp != tt
            })
            .collect_vec();
        perimeter += edges.len() as u64;
        todo.extend(ns.into_iter().filter(|(x, y)| grid.get(*x, *y) == t0));
        edges.into_iter().for_each(|e| {
            perimeter_edges.entry(e).or_insert(vec![]).push((x, y));
        });
    }

    fn merge_vertical_edges(mut acc: Vec<Vec<(i64, i64)>>, e: (i64, i64)) -> Vec<Vec<(i64, i64)>> {
        if acc.is_empty() {
            acc.push(vec![e]);
        } else {
            let e2 = acc.last().unwrap().last().unwrap();
            if e2.0 == e.0 && e2.1 == e.1 - 1 {
                acc.last_mut().unwrap().push(e);
            } else {
                acc.push(vec![e]);
            }
        }
        acc
    }
    fn merge_horizontal_edges(
        mut acc: Vec<Vec<(i64, i64)>>,
        e: (i64, i64),
    ) -> Vec<Vec<(i64, i64)>> {
        if acc.is_empty() {
            acc.push(vec![e]);
        } else {
            let e2 = acc.last().unwrap().last().unwrap();
            if e2.1 == e.1 && e2.0 == e.0 - 1 {
                acc.last_mut().unwrap().push(e);
            } else {
                acc.push(vec![e]);
            }
        }
        acc
    }

    let mut left_edges = perimeter_edges.get(&(-1, 0)).unwrap().clone();
    left_edges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let left_edges = left_edges.into_iter().fold(vec![], merge_vertical_edges);

    let mut right_edges = perimeter_edges.get(&(1, 0)).unwrap().clone();
    right_edges.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    let right_edges = right_edges.into_iter().fold(vec![], merge_vertical_edges);

    let mut up_edges = perimeter_edges.get(&(0, -1)).unwrap().clone();
    up_edges.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let up_edges = up_edges.into_iter().fold(vec![], merge_horizontal_edges);

    let mut down_edges = perimeter_edges.get(&(0, 1)).unwrap().clone();
    down_edges.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let down_edges = down_edges.into_iter().fold(vec![], merge_horizontal_edges);

    let sides = (left_edges.len() + right_edges.len() + up_edges.len() + down_edges.len()) as u64;

    Some((area, perimeter, sides))
}

fn main() {
    let mut grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let regions = iproduct!(0..grid.height, 0..grid.width,)
        .filter_map(|(y, x)| trace_region(&mut grid, x, y))
        .collect_vec();
    let r1 = regions.iter().map(|(a, p, _)| a * p).sum::<u64>();
    println!("{}", r1);

    let r2 = regions.iter().map(|(a, _, s)| a * s).sum::<u64>();
    println!("{}", r2);
}
