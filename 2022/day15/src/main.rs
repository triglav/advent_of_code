use std::{cmp::max, collections::HashSet, io};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn parse_line(l: &String) -> (Coord, Coord) {
    let t: Vec<_> = l
        .split(':')
        .flat_map(|s| s.split(','))
        .flat_map(|s| s.split('='))
        .filter(|s| s.chars().any(|c| c.is_digit(10)))
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    (
        Coord {
            x: *t.get(0).unwrap(),
            y: *t.get(1).unwrap(),
        },
        Coord {
            x: *t.get(2).unwrap(),
            y: *t.get(3).unwrap(),
        },
    )
}

struct Sensor {
    coord: Coord,
    distance: i32,
}

fn sensor_distance(sensor: Coord, beacon: Coord) -> i32 {
    (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs()
}

fn covered_range(s: &Sensor, y: i32) -> Option<(i32, i32)> {
    let dy = (s.coord.y - y).abs();
    let dx = s.distance - dy;
    if dx > 0 {
        Some((s.coord.x - dx, s.coord.x + dx))
    } else {
        None
    }
}

// "sorted" ranges
fn merge_ranges(a: &(i32, i32), b: &(i32, i32)) -> Vec<(i32, i32)> {
    // overlap
    if a.1 >= b.0 || a.1 + 1 == b.0 {
        vec![(a.0, max(a.1, b.1))]
    } else {
        vec![*a, *b]
    }
}

fn main() {
    let lines = io::stdin().lines().map(|f| f.unwrap());

    let sensors_and_beacons: Vec<_> = lines.map(|l| parse_line(&l)).collect();
    let sensors: Vec<_> = sensors_and_beacons
        .iter()
        .map(|(s, b)| Sensor {
            coord: *s,
            distance: sensor_distance(*s, *b),
        })
        .collect();

    let row1 = 2000000;
    // let row1 = 10;
    let beacons1 = sensors_and_beacons
        .iter()
        .fold(HashSet::<Coord>::new(), |mut a, (_s, b)| {
            a.insert(*b);
            a
        })
        .iter()
        .filter(|b| b.y == row1)
        .count();
    let mut covered1: Vec<_> = sensors
        .iter()
        .map(|s| covered_range(&s, row1))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();
    covered1.sort_by(|a, b| {
        let c = a.0.cmp(&b.0);
        match c {
            std::cmp::Ordering::Equal => a.1.cmp(&b.1),
            _ => c,
        }
    });
    let covered_merged1 = covered1.iter().fold(vec![], |mut a, c| {
        if a.is_empty() {
            a.push(*c);
        } else {
            let c0 = a.pop().unwrap();
            let v = merge_ranges(&c0, c);
            a.extend(v);
        }
        a
    });
    let covered_count1 = covered_merged1.iter().map(|(l, r)| r - l + 1).sum::<i32>();
    let r1 = (covered_count1 as usize) - beacons1;
    println!("{}", r1);
}
