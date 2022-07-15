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

    fn check_slope(map: &Vec<Vec<bool>>, slope: (usize, usize)) -> usize {
        let map_y = map.len();
        let map_x = map[0].len();
        let (slope_x, slope_y) = slope;
        (0..(map.len() / slope_y))
            .scan((0, 0), |pos, i| {
                *pos = (slope_x * i, slope_y * i);
                Some(*pos)
            })
            .filter(|(x, y)| map[*y % map_y][*x % map_x])
            .count()
    }

    let r = check_slope(&map, (3, 1));
    println!("{}", r);

    let r2 = [
        check_slope(&map, (1, 1)),
        r,
        check_slope(&map, (5, 1)),
        check_slope(&map, (7, 1)),
        check_slope(&map, (1, 2)),
    ]
    .into_iter()
    .reduce(|a, i| a * i)
    .unwrap();
    println!("{}", r2);
}
