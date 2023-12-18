use core::fmt;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    io,
};

enum Operation {
    Right(i32),
    Down(i32),
    Left(i32),
    Up(i32),
}

fn parse(line: &str) -> Operation {
    let t = line.split(' ').collect::<Vec<&str>>();
    let l = t[1].parse::<i32>().unwrap();
    match t[0] {
        "R" => Operation::Right(l),
        "D" => Operation::Down(l),
        "L" => Operation::Left(l),
        "U" => Operation::Up(l),
        _ => panic!("Unknown operation"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Trench,
    Left,
    Right,
}

struct Trench {
    data: HashMap<(i32, i32), TileType>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,

    pos_x: i32,
    pos_y: i32,
}

impl fmt::Display for Trench {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let c = match self.data.get(&(x, y)) {
                    Some(TileType::Trench) => '#',
                    Some(TileType::Left) => 'L',
                    Some(TileType::Right) => 'R',
                    None => '.',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn rotate_left(x: i32, y: i32) -> (i32, i32) {
    (-y, x)
}

fn rotate_right(x: i32, y: i32) -> (i32, i32) {
    (y, -x)
}

impl Trench {
    fn new() -> Self {
        Self {
            data: HashMap::from([((0, 0), TileType::Trench)]),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn update_bounds(&mut self, x: i32, y: i32) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    fn dig(&mut self, o: Operation) {
        let (l, (dx, dy)) = match o {
            Operation::Right(l) => (l, (1, 0)),
            Operation::Down(l) => (l, (0, 1)),
            Operation::Left(l) => (l, (-1, 0)),
            Operation::Up(l) => (l, (0, -1)),
        };
        for _ in 0..l {
            self.pos_x += dx;
            self.pos_y += dy;
            self.data.insert((self.pos_x, self.pos_y), TileType::Trench);
            self.update_bounds(self.pos_x, self.pos_y);

            let (lx, ly) = rotate_left(dx, dy);
            let (rx, ry) = rotate_right(dx, dy);

            let lt = self.data.get(&(self.pos_x + lx, self.pos_y + ly));
            if lt.is_none() {
                self.data
                    .insert((self.pos_x + lx, self.pos_y + ly), TileType::Left);
                self.update_bounds(self.pos_x + lx, self.pos_y + ly);
            }
            let rt = self.data.get(&(self.pos_x + rx, self.pos_y + ry));
            if rt.is_none() {
                self.data
                    .insert((self.pos_x + rx, self.pos_y + ry), TileType::Right);
                self.update_bounds(self.pos_x + rx, self.pos_y + ry);
            }
        }
    }

    fn get_neighbours(&self, x: i32, y: i32) -> Vec<(i32, i32)> {
        vec![
            (x, (y - 1).max(self.min_y)),
            ((x + 1).min(self.max_x), y),
            (x, (y + 1).min(self.max_x)),
            ((x - 1).max(self.min_x), y),
        ]
        .into_iter()
        .filter(|(x2, y2)| x2 != &x || y2 != &y)
        .collect::<Vec<_>>()
    }

    fn fill(&mut self) {
        let zz = self.data.iter().filter_map(
            |(&c, &t)| {
                if t != TileType::Trench {
                    Some(c)
                } else {
                    None
                }
            },
        );
        let mut todo = VecDeque::from(zz.collect::<Vec<_>>());
        while let Some((x, y)) = todo.pop_front() {
            let &t = self.data.get(&(x, y)).expect("Missing tile");
            self.get_neighbours(x, y).into_iter().for_each(|(x, y)| {
                if let Entry::Vacant(e) = self.data.entry((x, y)) {
                    e.insert(t);
                    todo.push_back((x, y));
                }
            });
        }
    }

    fn volume(&self) -> usize {
        let edge = (self.min_x..=self.max_x)
            .map(|x| self.data.get(&(x, self.min_y)).unwrap())
            .find(|&&t| t != TileType::Trench)
            .unwrap();
        self.data.values().filter(|&&t| t != *edge).count()
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let operations = lines.iter().map(|l| parse(l.as_str()));
    let mut trench = operations.fold(Trench::new(), |mut t, o| {
        t.dig(o);
        t
    });
    trench.fill();
    let r1 = trench.volume();
    println!("{}", r1);
}
