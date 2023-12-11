use std::{collections::HashSet, io};

use itertools::{iproduct, Itertools};

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let is_row_expandable = (0..height)
        .filter(|&row| (0..width).all(|col| grid[row][col] == '.'))
        .collect::<HashSet<_>>();
    let is_col_expandable = (0..width)
        .filter(|&col| (0..height).all(|row| grid[row][col] == '.'))
        .collect::<HashSet<_>>();

    let galaxies = iproduct!(0..width, 0..height)
        .filter(|(col, row)| grid[*row][*col] == '#')
        .collect::<Vec<_>>();

    let r1 = galaxies
        .iter()
        .combinations(2)
        .map(|c| {
            assert_eq!(c.len(), 2);
            let x1 = c[0].0.min(c[1].0);
            let x2 = c[0].0.max(c[1].0);
            let y1 = c[0].1.min(c[1].1);
            let y2 = c[0].1.max(c[1].1);

            let expandable_x = (x1..=x2)
                .filter(|&x| is_col_expandable.contains(&x))
                .count();
            let expandable_y = (y1..=y2)
                .filter(|&y| is_row_expandable.contains(&y))
                .count();

            let x = x2 - x1 + expandable_x;
            let y = y2 - y1 + expandable_y;
            x + y
        })
        .sum::<usize>();
    println!("{}", r1);
}
