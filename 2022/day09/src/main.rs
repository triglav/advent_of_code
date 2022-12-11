use std::{collections::HashSet, io};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl Default for Coord {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone)]
struct Rope {
    head: Coord,
    tail: Coord,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: Default::default(),
        }
    }
}

impl Rope {
    fn move_head(&mut self, direction: &Coord) {
        self.head.x += direction.x;
        self.head.y += direction.y;
        self.adjust_tail_pos();
    }

    fn update_head(&mut self, pos: &Coord) {
        self.head.x = pos.x;
        self.head.y = pos.y;
        self.adjust_tail_pos();
    }

    fn adjust_tail_pos(&mut self) {
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

struct LargeRope {
    segments: Vec<Rope>,
}

impl LargeRope {
    fn move_head(&mut self, direction: &Coord) {
        let len = self.segments.len();
        let head = self.segments.get_mut(0);
        head.unwrap().move_head(direction);

        for i in 1..len {
            let t0 = self.segments.get(i - 1).unwrap().tail;
            let t = self.segments.get_mut(i).unwrap();
            t.update_head(&t0);
        }
    }

    fn tail(&self) -> Coord {
        self.segments.last().unwrap().tail
    }
}

fn main() {
    let mut rope = Rope::default();
    let mut rope2 = LargeRope {
        segments: vec![Rope::default(); 9],
    };

    let mut tail_positions = HashSet::<Coord>::new();
    let mut tail_positions2 = HashSet::<Coord>::new();

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

            rope2.move_head(&dir_vec);
            tail_positions2.insert(rope2.tail());
        }
    }
    let r1 = tail_positions.len();
    println!("{}", r1);

    let r2 = tail_positions2.len();
    println!("{}", r2);
}
