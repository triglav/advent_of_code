use std::{cmp::max, collections::HashMap, io, ops};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

type Shape = Vec<Coord>;

fn shape0() -> Shape {
    vec![
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
    ]
}

fn shape1() -> Shape {
    vec![
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(1, 2),
    ]
}

fn shape2() -> Shape {
    vec![
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(2, 1),
        Coord::new(2, 2),
    ]
}

fn shape3() -> Shape {
    vec![
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(0, 2),
        Coord::new(0, 3),
    ]
}

fn shape4() -> Shape {
    vec![
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
    ]
}

struct Rock {
    pos: Coord,
    shape: Shape,
}

impl Rock {
    fn new(pos: Coord, shape: usize) -> Self {
        Self {
            pos,
            shape: match shape {
                0 => shape0(),
                1 => shape1(),
                2 => shape2(),
                3 => shape3(),
                4 => shape4(),
                _ => panic!("Invalid shape type"),
            },
        }
    }
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

type Situation = (usize, usize, Vec<usize>);

struct Chamber<'a> {
    width: i32,
    height: usize,
    rock_count: usize,
    grid: Vec<Vec<bool>>,
    jet_pattern: &'a Vec<Jet>,
    next_jet_idx: usize,

    height_per_column: Vec<usize>,
}

impl<'a> Chamber<'a> {
    fn new(width: i32, jet_pattern: &'a Vec<Jet>) -> Self {
        Self {
            width,
            height: 0,
            rock_count: 0,
            grid: vec![],
            jet_pattern,
            next_jet_idx: 0,
            height_per_column: vec![0; width as usize],
        }
    }

    fn enlarge_grid(&mut self, height: usize) {
        for _ in self.grid.len()..=height {
            self.grid.push(vec![false; self.width as usize]);
        }
    }

    fn spawn_rock(&mut self) -> (Rock, Situation) {
        let rock_shape = self.rock_count % 5;
        let pos = Coord::new(2, self.height as i32 + 3);
        let rock = Rock::new(pos, rock_shape);
        self.rock_count += 1;
        self.enlarge_grid(pos.y as usize + 3);

        let top_offsets = self.get_top_offsets(pos.y as usize);
        let situation = (rock_shape, self.next_jet_idx, top_offsets);
        (rock, situation)
    }

    fn collides(&self, pos: Coord, shape: &Shape) -> bool {
        shape.iter().map(|offset| *offset + pos).any(|c| {
            if c.x < 0 || c.x >= self.width || c.y < 0 {
                return true;
            }
            *self
                .grid
                .get(c.y as usize)
                .unwrap()
                .get(c.x as usize)
                .unwrap()
        })
    }

    fn next_jet_direction(&mut self) -> Coord {
        let p = self.jet_pattern.get(self.next_jet_idx).unwrap();
        self.next_jet_idx = (self.next_jet_idx + 1) % self.jet_pattern.len();
        match *p {
            Jet::Left => Coord::new(-1, 0),
            Jet::Right => Coord::new(1, 0),
        }
    }

    // true if moved
    fn move_rock(&self, rock: &mut Rock, dir: Coord) -> bool {
        let p2 = rock.pos + dir;
        if !self.collides(p2, &rock.shape) {
            rock.pos = p2;
            return true;
        }
        false
    }

    fn push_rock_by_jet(&mut self, rock: &mut Rock) -> bool {
        let dir = self.next_jet_direction();
        self.move_rock(rock, dir)
    }

    fn apply_gravity(&mut self, rock: &mut Rock) -> bool {
        let dir = Coord::new(0, -1);
        self.move_rock(rock, dir)
    }

    fn lock_in(&mut self, rock: Rock) {
        for c in rock.shape.iter().map(|offset| *offset + rock.pos) {
            let p = self
                .grid
                .get_mut(c.y as usize)
                .unwrap()
                .get_mut(c.x as usize)
                .unwrap();
            assert_eq!(*p, false);
            *p = true;
            self.height = max(self.height, c.y as usize + 1);
            let column_height = self.height_per_column.get_mut(c.x as usize).unwrap();
            *column_height = max(*column_height, c.y as usize + 1);
        }
    }

    fn simulate_rock(&mut self) -> Situation {
        let (mut rock, situation) = self.spawn_rock();
        loop {
            self.push_rock_by_jet(&mut rock);
            if !self.apply_gravity(&mut rock) {
                self.lock_in(rock);
                break;
            }
        }
        situation
    }

    fn get_top_offsets(&self, y: usize) -> Vec<usize> {
        self.height_per_column.iter().map(|h| y - h).collect()
    }
}

fn main() {
    let mut line = String::new();
    let r = io::stdin().read_line(&mut line);
    if r.is_err() {
        panic!("Invalid input");
    }

    let jet_pattern: Vec<_> = line
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid pattern type `{}`", c),
        })
        .collect();
    let mut c = Chamber::new(7, &jet_pattern);
    for _ in 0..2022 {
        c.simulate_rock();
    }
    let r1 = c.height;
    println!("{}", r1);

    let mut c2 = Chamber::new(7, &jet_pattern);
    let mut previous_situations = HashMap::<Situation, (usize, usize)>::new();
    let ((rc0, h0), (rc1, h1)) = loop {
        let s = c2.simulate_rock();
        if let Some(prev) = previous_situations.insert(s, (c2.rock_count, c2.height)) {
            break (prev, (c2.rock_count, c2.height));
        }
    };

    let rocks2 = 1000000000000_usize;

    let d_r = rc1 - rc0;
    let d_h = h1 - h0;

    let ratio = (rocks2 - rc0) / d_r;
    let h2 = ratio * d_h;

    let rc3 = (rocks2 - rc0) % d_r;
    for _ in 0..rc3 {
        c2.simulate_rock();
    }
    let h3 = c2.height - h1;

    let r2 = h0 + h2 + h3;
    println!("{}", r2);
}
