use std::{io, ops};

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Option<Tile>>,
}

impl Board {
    fn idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn tile(&self, x: usize, y: usize) -> &Option<Tile> {
        let i = self.idx(x, y);
        self.tiles.get(i).unwrap()
    }

    fn tile_mut(&mut self, x: usize, y: usize) -> &mut Option<Tile> {
        let i = self.idx(x, y);
        self.tiles.get_mut(i).unwrap()
    }

    fn find_initial_position(&self) -> Coord {
        let y = 0;
        let x = self
            .tiles
            .iter()
            .position(|o| match o {
                Some(Tile::Empty) => true,
                _ => false,
            })
            .unwrap();
        Coord::new(x as i32, y)
    }

    fn wrap_pos(&self, pos: Coord) -> Coord {
        fn wrap(v: i32, top: i32) -> i32 {
            if v >= top {
                0
            } else if v < 0 {
                top - 1
            } else {
                v
            }
        }
        Coord {
            x: wrap(pos.x, self.width as i32),
            y: wrap(pos.y, self.height as i32),
        }
    }

    fn find_next_position(&self, pos: Coord, dir: Coord) -> Option<Coord> {
        fn advance(this: &Board, pos: Coord, dir: Coord) -> Coord {
            this.wrap_pos(Coord {
                x: pos.x + dir.x,
                y: pos.y - dir.y,
            })
        }
        let mut next_pos = advance(self, pos, dir);
        loop {
            match self.tile(next_pos.x as usize, next_pos.y as usize) {
                Some(Tile::Empty) => break Some(next_pos),
                Some(Tile::Wall) => break None,
                None => {
                    next_pos = advance(self, next_pos, dir);
                    continue;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Path {
    TurnRight,
    TurnLeft,
    Walk(u32),
}

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

struct Actor {
    pos: Coord,
    dir: Coord,
}

impl Actor {
    fn turn_right(&mut self) {
        (self.dir.x, self.dir.y) = (self.dir.y, -self.dir.x);
    }

    fn turn_left(&mut self) {
        (self.dir.x, self.dir.y) = (-self.dir.y, self.dir.x);
    }

    fn walk(&mut self, mut distance: u32, b: &Board) {
        while distance > 0 {
            if let Some(next_pos) = b.find_next_position(self.pos, self.dir) {
                self.pos = next_pos;
                distance -= 1;
            } else {
                break;
            }
        }
    }

    fn act(&mut self, p: Path, b: &Board) {
        match p {
            Path::TurnRight => self.turn_right(),
            Path::TurnLeft => self.turn_left(),
            Path::Walk(distance) => self.walk(distance, b),
        }
    }
}

fn parse_board(lines: &Vec<String>) -> Board {
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    let height = lines.len();
    let mut board = Board {
        width,
        height,
        tiles: vec![None; width * height],
    };

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let t = match c {
                '.' => Some(Tile::Empty),
                '#' => Some(Tile::Wall),
                _ => None,
            };
            *board.tile_mut(x, y) = t;
        }
    }
    board
}

fn parse_path(line: &String) -> Vec<Path> {
    let mut r = Vec::<Path>::new();
    let mut s = String::new();
    for c in line.chars() {
        match c {
            'R' => {
                if !s.is_empty() {
                    r.push(Path::Walk(s.parse::<u32>().unwrap()));
                    s.clear();
                }
                r.push(Path::TurnRight);
            }
            'L' => {
                if !s.is_empty() {
                    r.push(Path::Walk(s.parse::<u32>().unwrap()));
                    s.clear();
                }
                r.push(Path::TurnLeft);
            }
            '0'..='9' => s.push(c),
            _ => panic!("Unexpected path"),
        };
    }
    if !s.is_empty() {
        r.push(Path::Walk(s.parse::<u32>().unwrap()));
    }
    r
}

fn main() {
    let mut lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let path_str = lines.pop().unwrap();
    lines.pop();

    let board = parse_board(&lines);
    let path = parse_path(&path_str);
    let mut actor = Actor {
        pos: board.find_initial_position(),
        dir: Coord::new(1, 0),
    };
    for s in path {
        actor.act(s, &board);
    }

    let row = actor.pos.y + 1;
    let column = actor.pos.x + 1;
    let facing = match (actor.dir.x, actor.dir.y) {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => panic!(),
    };
    let r1 = 1000 * row + 4 * column + facing;
    println!("{}", r1);
}
