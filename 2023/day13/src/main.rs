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

    fn fix_smudge(&self) -> impl Iterator<Item = Self> + '_ {
        (0..self.grid.len()).map(|i| {
            let mut g2 = self.grid.clone();
            g2[i] = match g2[i] {
                '#' => '.',
                '.' => '#',
                _ => panic!("invalid char"),
            };
            Grid {
                width: self.width,
                height: self.height,
                grid: g2,
            }
        })
    }
}

#[derive(Debug)]
struct Pattern {
    pub horizontal_bitmap: Vec<u64>,
    pub vertical_bitmap: Vec<u64>,
}

fn find_reflection_detail(bitmaps: &[u64]) -> Vec<usize> {
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
    reflections
}

impl Pattern {
    fn from(grid: &Grid) -> Self {
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

    fn find_reflection(&self) -> (Option<usize>, Option<usize>) {
        let h = find_reflection_detail(&self.horizontal_bitmap);
        let v = find_reflection_detail(&self.vertical_bitmap);
        assert!(
            h.len() + v.len() <= 1,
            "too many reflections {:?} {:?}",
            h,
            v
        );
        (h.into_iter().next(), v.into_iter().next())
    }

    fn find_fixed_reflection(
        &self,
        r0: (Option<usize>, Option<usize>),
    ) -> (Option<usize>, Option<usize>) {
        let h = find_reflection_detail(&self.horizontal_bitmap);
        let v = find_reflection_detail(&self.vertical_bitmap);

        let h = h
            .into_iter()
            .filter(|&h| h != r0.0.unwrap_or(0))
            .collect::<Vec<_>>();
        let v = v
            .into_iter()
            .filter(|&v| v != r0.1.unwrap_or(0))
            .collect::<Vec<_>>();
        assert!(
            h.len() + v.len() <= 1,
            "too many reflections {:?} {:?}",
            h,
            v
        );
        (h.into_iter().next(), v.into_iter().next())
    }
}

fn main() {
    let grids = io::stdin()
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
        .map(Grid::from)
        .collect::<Vec<_>>();

    let r1 = grids
        .iter()
        .map(Pattern::from)
        .map(|p| match p.find_reflection() {
            (Some(h), None) => h as u64 * 100,
            (None, Some(v)) => v as u64,
            _ => panic!("invalid pattern"),
        })
        .sum::<u64>();
    println!("{:?}", r1);

    let r2 = grids
        .iter()
        .map(|g| {
            let r0 = Pattern::from(g).find_reflection();

            let patterns = g.fix_smudge().map(|g| Pattern::from(&g));
            let reflections = patterns
                .filter_map(|p| {
                    let r = p.find_fixed_reflection(r0);
                    // println!("{:?}", r);
                    match r {
                        (Some(h), None) => Some(h as u64 * 100),
                        (None, Some(v)) => Some(v as u64),
                        (None, None) => None,
                        _ => panic!("invalid pattern"),
                    }
                })
                .unique()
                .collect::<Vec<_>>();
            assert!(reflections.len() <= 1, "too many ? {:?}", reflections);
            reflections.into_iter().next().unwrap()
        })
        .sum::<u64>();
    println!("{:?}", r2);
}
