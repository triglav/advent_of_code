use std::io;

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Operation {
    direction: Direction,
    length: i64,
}

fn parse(line: &str) -> (Operation, Operation) {
    let t = line.split(' ').collect::<Vec<&str>>();
    let l = t[1].parse::<i64>().unwrap();
    let d1 = match t[0] {
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "U" => Direction::Up,
        _ => panic!("Unknown operation"),
    };
    let o1 = Operation {
        direction: d1,
        length: l,
    };

    let l = t[2][2..7]
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .fold(0, |acc, d| acc * 16 + d) as i64;
    let d2 = match t[2][7..=7].parse::<i32>().unwrap() {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => panic!("Unknown operation"),
    };
    let o2 = Operation {
        direction: d2,
        length: l,
    };
    (o1, o2)
}

fn shoelace(operations: &[Operation]) -> u64 {
    let area = operations
        .iter()
        .scan((0, 0), |p, &o| {
            match o.direction {
                Direction::Right => p.0 += o.length,
                Direction::Down => p.1 -= o.length,
                Direction::Left => p.0 -= o.length,
                Direction::Up => p.1 += o.length,
            };
            Some(*p)
        })
        .tuple_windows::<(_, _)>()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - y1 * x2)
        .sum::<i64>()
        .unsigned_abs()
        / 2;
    let perimeter = operations.iter().map(|o| o.length as u64).sum::<u64>();
    area + perimeter / 2 + 1
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let operations = lines.iter().map(|l| parse(l.as_str())).collect::<Vec<_>>();

    let o1 = operations.iter().map(|o| o.0).collect::<Vec<_>>();
    let o2 = operations.iter().map(|o| o.1).collect::<Vec<_>>();

    let r1 = shoelace(&o1);
    println!("{}", r1);

    let r2 = shoelace(&o2);
    println!("{}", r2);
}
