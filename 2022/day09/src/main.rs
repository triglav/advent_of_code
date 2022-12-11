use std::{collections::HashSet, io};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

struct Rope {
    head: Coord,
    tail: Coord,
}

impl Rope {
    fn move_head(&mut self, direction: &Coord) {
        self.head.x += direction.x;
        self.head.y += direction.y;

        let dx = self.head.x.abs_diff(self.tail.x);
        let dy = self.head.y.abs_diff(self.tail.y);
        if dx <= 1 && dy <= 1 {
            return;
        }

        if self.head.x == self.tail.x {
            self.tail.y += (self.head.y - self.tail.y).signum();
        } else if self.head.y == self.tail.y {
            self.tail.x += (self.head.x - self.tail.x).signum();
        } else {
            self.tail.y += (self.head.y - self.tail.y).signum();
            self.tail.x += (self.head.x - self.tail.x).signum();
        }
    }
}

fn main() {
    let mut rope = Rope {
        head: Coord { x: 0, y: 0 },
        tail: Coord { x: 0, y: 0 },
    };

    let mut tail_positions = HashSet::<Coord>::new();

    for l in io::stdin().lines() {
        let s = l.unwrap();
        let mut t = s.split(' ');
        let dir_code = t.next().unwrap();
        let distance = t.next().unwrap().parse::<u32>().unwrap();

        let dir_vec = match dir_code {
            "U" => Coord { x: 0, y: 1 },
            "D" => Coord { x: 0, y: -1 },
            "R" => Coord { x: 1, y: 0 },
            "L" => Coord { x: -1, y: 0 },
            _ => panic!("Invalid direction code {}", dir_code),
        };
        for _ in 0..distance {
            rope.move_head(&dir_vec);
            tail_positions.insert(rope.tail);
        }
    }
    let r1 = tail_positions.len();
    println!("{}", r1);
}
