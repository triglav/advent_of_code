use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
    io,
    ops::{Add, AddAssign, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Blizzard {
    pos: Coord,
    dir: Coord,
}

impl Blizzard {
    fn new(pos: Coord, dir: Coord) -> Self {
        Self { pos, dir }
    }
}

struct Valley {
    width: i32,
    height: i32,
    start: Coord,
    goal: Coord,
    blizzards: Vec<Blizzard>,
}

impl Valley {
    fn tick(&mut self) -> HashSet<Coord> {
        let mut blizz_positions = HashSet::<Coord>::new();
        for b in self.blizzards.iter_mut() {
            b.pos += b.dir;
            if b.pos.x == self.width - 1 {
                b.pos.x = 1;
            } else if b.pos.x == 0 {
                b.pos.x = self.width - 2;
            }
            if b.pos.y == self.height - 1 {
                b.pos.y = 1;
            } else if b.pos.y == 0 {
                b.pos.y = self.height - 2;
            }
            blizz_positions.insert(b.pos);
        }
        blizz_positions
    }
}

fn parse_valley(lines: &Vec<String>) -> Valley {
    let first_line = lines.first().unwrap();
    let last_line = lines.last().unwrap();

    let width = first_line.len() as i32;
    let height = lines.len() as i32;

    let start = Coord::new(first_line.chars().position(|c| c == '.').unwrap() as i32, 0);
    let goal = Coord::new(
        last_line.chars().position(|c| c == '.').unwrap() as i32,
        (height - 1) as i32,
    );
    let mut blizzards = vec![];

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if let Some(dir) = match c {
                '>' => Some(Coord::new(1, 0)),
                '<' => Some(Coord::new(-1, 0)),
                '^' => Some(Coord::new(0, -1)),
                'v' => Some(Coord::new(0, 1)),
                _ => None,
            } {
                blizzards.push(Blizzard::new(Coord::new(x as i32, y as i32), dir));
            }
        }
    }
    Valley {
        width,
        height,
        start,
        goal,
        blizzards,
    }
}

fn get_available_positions(pos: Coord, v: &Valley, blizz_positions: &HashSet<Coord>) -> Vec<Coord> {
    let mut r = vec![];
    for (x, y) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
        if x == 0 && y == 0 {
            continue;
        }
        let c = Coord::new(pos.x + x, pos.y + y);
        if c.x <= 0 || c.y <= 0 || c.x >= v.width - 1 || c.y >= v.height - 1 {
            if c != v.start && c != v.goal {
                continue;
            }
        }
        if blizz_positions.contains(&c) {
            continue;
        }
        r.push(c);
    }
    if !blizz_positions.contains(&pos) {
        r.push(pos);
    }
    r
}

fn make_key2(blizz_positions: &HashSet<Coord>) -> String {
    let mut v: Vec<_> = blizz_positions.iter().collect();
    v.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    format!("{:?}", v)
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    pos: Coord,
    minute: usize,
    history: Vec<Coord>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.minute.cmp(&self.minute)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_all_states(valley: &mut Valley) -> Vec<HashSet<Coord>> {
    let initial_blizz: HashSet<_> = valley.blizzards.iter().map(|b| b.pos).collect();
    let key = make_key2(&initial_blizz);

    let mut seen = HashMap::<String, usize>::new();
    seen.insert(key, 0);
    let mut blizz_states = vec![initial_blizz];

    let mut minutes = 1;

    loop {
        let blizz_positions = valley.tick();
        let key = make_key2(&blizz_positions);
        if let Some(_) = seen.insert(key, minutes) {
            break;
        }
        blizz_states.push(blizz_positions);
        minutes += 1;
    }
    blizz_states
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let mut valley = parse_valley(&lines);

    let mut todo = BinaryHeap::<State>::new();
    todo.push(State {
        pos: valley.start,
        minute: 0,
        history: vec![valley.start],
    });
    let blizz_states = find_all_states(&mut valley);

    let mut seen = HashSet::<(Coord, usize)>::new();
    seen.insert((valley.start, 0));

    let mut r1 = None;
    while let Some(State {
        pos,
        minute,
        history,
    }) = todo.pop()
    {
        if pos == valley.goal {
            r1 = Some(minute);
            break;
        }
        let next_minute = minute + 1;
        let next_state_idx = next_minute % blizz_states.len();
        let blizz_positions = blizz_states.get(next_state_idx).unwrap();
        let next_positions = get_available_positions(pos, &valley, &blizz_positions);

        for np in next_positions {
            if seen.insert((np, next_state_idx)) {
                let mut h2 = history.clone();
                h2.push(np);
                todo.push(State {
                    pos: np,
                    minute: next_minute,
                    history: h2,
                });
            }
        }
    }
    println!("{}", r1.unwrap());
}
