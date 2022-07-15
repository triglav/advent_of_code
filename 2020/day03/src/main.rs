use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let map = lines
        .iter()
        .map(|l| {
            l.as_bytes()
                .iter()
                .map(|c| *c as char == '#')
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    let map_y = map.len();
    let map_x = map[0].len();

    let (slope_x, slope_y) = (3, 1);
    let r = (0..map.len())
        .scan((0, 0), |pos, i| {
            *pos = (slope_x * i, slope_y * i);
            Some(*pos)
        })
        .filter(|(x, y)| map[*y % map_y][*x % map_x])
        .count();
    println!("{}", r);
}
