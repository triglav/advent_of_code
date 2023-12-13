use std::io;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<char>,
}

impl Grid {
    fn from(lines: Vec<String>) -> Self {
        let width = lines[0].len();
        let height = lines.len();
        let grid = lines.iter().flat_map(|l| l.chars()).collect();
        Self {
            width,
            height,
            grid,
        }
    }

    fn get_row_bitmap(&self, row: usize) -> u64 {
        (0..self.width).fold(0, |acc, i| {
            let c = self.grid[row * self.width + i];
            if c == '#' {
                acc | (1 << i)
            } else {
                acc
            }
        })
    }

    fn get_col_bitmap(&self, col: usize) -> u64 {
        (0..self.height).fold(0, |acc, i| {
            let c = self.grid[i * self.width + col];
            if c == '#' {
                acc | (1 << i)
            } else {
                acc
            }
        })
    }
}

#[derive(Debug)]
struct Pattern {
    pub horizontal_bitmap: Vec<u64>,
    pub vertical_bitmap: Vec<u64>,
}

impl Pattern {
    fn from_grid(grid: Grid) -> Self {
        let horizontal_bitmap = (0..grid.height)
            .map(|i| grid.get_row_bitmap(i))
            .collect::<Vec<_>>();
        let vertical_bitmap = (0..grid.width)
            .map(|i| grid.get_col_bitmap(i))
            .collect::<Vec<_>>();
        Self {
            horizontal_bitmap,
            vertical_bitmap,
        }
    }

    fn from(lines: Vec<String>) -> Self {
        let grid = Grid::from(lines);
        Self::from_grid(grid)
    }

    fn find_reflection(&self) -> (Option<usize>, Option<usize>) {
        fn find_reflection_detail(bitmaps: &[u64]) -> Option<usize> {
            let reflections = bitmaps
                .iter()
                .enumerate()
                .tuple_windows()
                .filter_map(|((i, a), (_, b))| if a == b { Some(i + 1) } else { None })
                .filter(|&c| {
                    let l = &bitmaps[0..c];
                    let r = &bitmaps[c..];
                    let is_perfect_reflection = l.iter().rev().zip(r.iter()).all(|(a, b)| a == b);
                    is_perfect_reflection
                })
                .collect::<Vec<usize>>();
            assert!(reflections.len() <= 1);
            reflections.into_iter().next()
        }

        let h = find_reflection_detail(&self.horizontal_bitmap);
        let v = find_reflection_detail(&self.vertical_bitmap);
        assert_ne!(h.is_some(), v.is_some());
        (h, v)
    }
}

fn main() {
    let r1 = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .fold(vec![vec![]], |mut a, b| {
            if !b.is_empty() {
                a.last_mut().unwrap().push(b);
            } else {
                a.push(vec![])
            }
            a
        })
        .into_iter()
        .map(Pattern::from)
        .map(|p| match p.find_reflection() {
            (Some(h), None) => h as u64 * 100,
            (None, Some(v)) => v as u64,
            _ => panic!("invalid pattern"),
        })
        .sum::<u64>();
    println!("{:?}", r1);
}
