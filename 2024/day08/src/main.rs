use std::collections::HashSet;
use std::{collections::HashMap, fmt, io};

use itertools::iproduct;
use itertools::Itertools;

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

fn get_antinodes(a: (i64, i64), b: (i64, i64)) -> Vec<(i64, i64)> {
    let ab = (b.0 - a.0, b.1 - a.1);
    let c = (a.0 - ab.0, a.1 - ab.1);
    let d = (b.0 + ab.0, b.1 + ab.1);
    vec![c, d]
}

fn get_antinodes2(a: (i64, i64), b: (i64, i64), width: i64, height: i64) -> Vec<(i64, i64)> {
    let verify = |x, y| {
        if x >= 0 && x < width && y >= 0 && y < height {
            Some((x, y))
        } else {
            None
        }
    };
    let ab = (b.0 - a.0, b.1 - a.1);
    let get_c = |i: i64| verify(a.0 - ab.0 * i, a.1 - ab.1 * i);
    let get_d = |i: i64| verify(b.0 + ab.0 * i, b.1 + ab.1 * i);

    let mut v = vec![];

    let mut ci = 0;
    while let Some(c) = get_c(ci) {
        v.push(c);
        ci += 1;
    }
    let mut di = 0;
    while let Some(d) = get_d(di) {
        v.push(d);
        di += 1;
    }
    v
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );

    let antennas = (0..grid.height)
        .flat_map(|y| (0..grid.width).map(move |x| (x, y)))
        .filter_map(|(x, y)| {
            let t = grid.get(x, y);
            if t == '.' {
                None
            } else {
                Some((x, y, t))
            }
        })
        .fold(HashMap::<char, Vec<_>>::new(), |mut acc, (x, y, t)| {
            acc.entry(t).or_default().push((x, y));
            acc
        });
    let antinodes = antennas
        .keys()
        .flat_map(|k| {
            antennas
                .get(k)
                .unwrap()
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| get_antinodes(*a, *b))
                .filter(|&(x, y)| x >= 0 && x < grid.width && y >= 0 && y < grid.height)
                .collect_vec()
        })
        .collect::<HashSet<_>>();
    let r1 = antinodes.len();
    println!("{}", r1);

    let antinodes2 = antennas
        .keys()
        .flat_map(|k| {
            antennas
                .get(k)
                .unwrap()
                .iter()
                .tuple_combinations()
                .flat_map(|(a, b)| get_antinodes2(*a, *b, grid.width, grid.height))
                .collect_vec()
        })
        .collect::<HashSet<_>>();
    let r2 = antinodes2.len();
    println!("{}", r2);
}
