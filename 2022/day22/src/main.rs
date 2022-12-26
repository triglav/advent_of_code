use std::{
    cell::RefCell,
    cmp::max,
    collections::{HashMap, VecDeque},
    fmt::Display,
    io,
    ops::{Add, Sub},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i32,
    y: i32,
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

    fn from_facing(facing: i32) -> Coord {
        match facing {
            0 => DIR_RIGHT,
            1 => DIR_DOWN,
            2 => DIR_LEFT,
            3 => DIR_UP,
            _ => panic!(),
        }
    }

    fn add_dir(&self, dir: Direction) -> Coord {
        Coord {
            x: self.x + dir.x,
            y: self.y - dir.y,
        }
    }

    fn turn_right(&self) -> Coord {
        Coord::new(self.y, -self.x)
    }

    fn turn_left(&self) -> Coord {
        Coord::new(-self.y, self.x)
    }

    fn turn_around(&self) -> Coord {
        self.turn_left().turn_left()
    }
}

// impl ops::Add<Coord> for Coord {
//     type Output = Coord;

//     fn add(self, other: Coord) -> Coord {
//         Coord {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

type Direction = Coord;
type Face = Coord;

const DIR_RIGHT: Direction = Coord { x: 1, y: 0 };
const DIR_DOWN: Direction = Coord { x: 0, y: -1 };
const DIR_LEFT: Direction = Coord { x: -1, y: 0 };
const DIR_UP: Direction = Coord { x: 0, y: 1 };

fn get_facing(dir: Direction) -> i32 {
    match dir {
        DIR_RIGHT => 0,
        DIR_DOWN => 1,
        DIR_LEFT => 2,
        DIR_UP => 3,
        _ => panic!(),
    }
}

fn wrap(v: i32, top: i32) -> i32 {
    if v >= top {
        0
    } else if v < 0 {
        top - 1
    } else {
        v
    }
}

fn turn_right(facing: i32) -> i32 {
    (facing + 1) % 4
}

fn turn_left(facing: i32) -> i32 {
    (facing + 3) % 4
}

fn turn_around(facing: i32) -> i32 {
    (facing + 2) % 4
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Wall,
}

struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Option<Tile>>,
    face_size: usize,
    face_max_x: i32,
    face_max_y: i32,
    faces: HashMap<Face, RefCell<HashMap<i32, (Face, i32)>>>,
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
        Coord {
            x: wrap(pos.x, self.width as i32),
            y: wrap(pos.y, self.height as i32),
        }
    }

    fn find_next_position_cube(&self, pos: Coord, dir: Direction) -> Option<(Coord, Direction)> {
        fn advance_cube(this: &Board, pos: Coord, dir: Direction) -> (Coord, Direction) {
            let fc = this.get_tile_face(pos);
            let pos2 = this.wrap_pos(pos.add_dir(dir));
            let fc2 = this.get_tile_face(pos2);
            if fc == fc2 {
                return (pos2, dir);
            }
            if this.faces.contains_key(&fc2) {
                return (pos2, dir);
            }
            let e = this.face_size as i32 - 1;
            let face = this.faces.get(&fc).unwrap().borrow();
            let facing = get_facing(dir);
            let o = pos - this.get_face_origin(fc);

            let (face2, facing2) = face.get(&facing).unwrap();
            let dir2 = Coord::from_facing(*facing2);
            let o2 = match dir {
                DIR_RIGHT => match dir2 {
                    DIR_RIGHT => Coord::new(e, e - o.y),
                    DIR_DOWN => Coord::new(o.y, e),
                    DIR_LEFT => Coord::new(0, o.y),
                    DIR_UP => Coord::new(e - o.y, 0),
                    _ => panic!(),
                },
                DIR_DOWN => match dir2 {
                    DIR_RIGHT => Coord::new(e, o.x),
                    DIR_DOWN => Coord::new(e - o.x, e),
                    DIR_LEFT => Coord::new(0, e - o.x),
                    DIR_UP => Coord::new(o.x, 0),
                    _ => panic!(),
                },
                DIR_LEFT => match dir2 {
                    DIR_RIGHT => Coord::new(e, o.y),
                    DIR_DOWN => Coord::new(e - o.y, e),
                    DIR_LEFT => Coord::new(0, e - o.y),
                    DIR_UP => Coord::new(o.y, 0),
                    _ => panic!(),
                },
                DIR_UP => match dir2 {
                    DIR_RIGHT => Coord::new(e, e - o.x),
                    DIR_DOWN => Coord::new(o.x, e),
                    DIR_LEFT => Coord::new(0, o.x),
                    DIR_UP => Coord::new(e - o.x, 0),
                    _ => panic!(),
                },
                _ => panic!(),
            };
            (this.get_face_origin(*face2) + o2, dir2.turn_around())
        }
        let (next_pos, next_dir) = advance_cube(self, pos, dir);
        match self.tile(next_pos.x as usize, next_pos.y as usize) {
            Some(Tile::Empty) => Some((next_pos, next_dir)),
            Some(Tile::Wall) => None,
            None => panic!("There has to be always a tile on cube"),
        }
    }

    fn find_next_position(&self, pos: Coord, dir: Direction) -> Option<Coord> {
        fn advance(this: &Board, pos: Coord, dir: Direction) -> Coord {
            this.wrap_pos(pos.add_dir(dir))
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

    fn get_tile_face(&self, pos: Coord) -> Face {
        Face {
            x: pos.x / (self.face_size as i32),
            y: pos.y / (self.face_size as i32),
        }
    }

    fn get_face_origin(&self, face: Face) -> Coord {
        Coord {
            x: face.x * self.face_size as i32,
            y: face.y * self.face_size as i32,
        }
    }

    fn map_faces(&mut self) {
        for (f, neighbours) in self.faces.iter() {
            for facing in 0..4 {
                let d = Coord::from_facing(facing);
                let f2 = f.add_dir(d);
                if self.faces.contains_key(&f2) {
                    neighbours
                        .borrow_mut()
                        .insert(facing, (f2, turn_around(facing)));
                }
            }
        }
        let mut todo: VecDeque<_> = self.faces.keys().copied().collect();
        while let Some(f0) = todo.pop_back() {
            for facing1 in 0..4 {
                let facing2 = turn_right(facing1);

                let neighbours = self.faces.get(&f0).unwrap().borrow();
                if let Some((f1, _)) = neighbours.get(&facing1) {
                    if let Some((f2, _)) = neighbours.get(&facing2) {
                        let mut fn1 = self.faces.get(f1).unwrap().borrow_mut();
                        let mut fn2 = self.faces.get(f2).unwrap().borrow_mut();

                        let f0f1_facing = fn1
                            .iter()
                            .find_map(|(f, (c, _))| if *c == f0 { Some(f) } else { None })
                            .unwrap();
                        let f1f2_facing = turn_left(*f0f1_facing);

                        let f0f2_facing = fn2
                            .iter()
                            .find_map(|(f, (c, _))| if *c == f0 { Some(f) } else { None })
                            .unwrap();
                        let f2f1_facing = turn_right(*f0f2_facing);

                        fn1.insert(f1f2_facing, (*f2, f2f1_facing));
                        fn2.insert(f2f1_facing, (*f1, f1f2_facing));
                    }
                }
            }
            let x = self.faces.get(&f0).unwrap().borrow().keys().count();
            if x < 4 {
                todo.push_front(f0);
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

struct Actor {
    pos: Coord,
    dir: Direction,
    cube: bool,
}

impl Actor {
    fn turn_right(&mut self) {
        self.dir = self.dir.turn_right();
    }

    fn turn_left(&mut self) {
        self.dir = self.dir.turn_left();
    }

    fn walk(&mut self, mut distance: u32, b: &Board) {
        while distance > 0 {
            if self.cube {
                if let Some((next_pos, next_dir)) = b.find_next_position_cube(self.pos, self.dir) {
                    self.pos = next_pos;
                    self.dir = next_dir;
                    distance -= 1;
                } else {
                    break;
                }
            } else {
                if let Some(next_pos) = { b.find_next_position(self.pos, self.dir) } {
                    self.pos = next_pos;
                    distance -= 1;
                } else {
                    break;
                }
            }
        }
    }

    fn act(&mut self, p: &Path, b: &Board) {
        match *p {
            Path::TurnRight => self.turn_right(),
            Path::TurnLeft => self.turn_left(),
            Path::Walk(distance) => self.walk(distance, b),
        }
    }
}

fn parse_board(lines: &Vec<String>, face_size: usize) -> Board {
    let width = lines.iter().map(|l| l.len()).max().unwrap();
    let height = lines.len();
    let mut board = Board {
        width,
        height,
        tiles: vec![None; width * height],
        face_size,
        face_max_x: 0,
        face_max_y: 0,
        faces: HashMap::new(),
    };

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let t = match c {
                '.' => Some(Tile::Empty),
                '#' => Some(Tile::Wall),
                _ => None,
            };
            *board.tile_mut(x, y) = t;
            if t.is_some() {
                let c = Coord::new(x as i32, y as i32);
                let face = board.get_tile_face(board.wrap_pos(c));
                if !board.faces.contains_key(&face) {
                    board.face_max_x = max(board.face_max_x, face.x);
                    board.face_max_y = max(board.face_max_y, face.y);
                    board.faces.insert(face, RefCell::new(HashMap::new()));
                }
            }
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

    let mut board = parse_board(&lines, 50);
    board.map_faces();
    let path = parse_path(&path_str);
    let mut actor = Actor {
        pos: board.find_initial_position(),
        dir: Direction::new(1, 0),
        cube: false,
    };
    let mut actor2 = Actor {
        pos: board.find_initial_position(),
        dir: Direction::new(1, 0),
        cube: true,
    };
    for s in path {
        actor.act(&s, &board);
        actor2.act(&s, &board);
    }

    let row1 = actor.pos.y + 1;
    let column1 = actor.pos.x + 1;
    let facing1 = get_facing(actor.dir);
    let r1 = 1000 * row1 + 4 * column1 + facing1;
    println!("{}", r1);

    let row2 = actor2.pos.y + 1;
    let column2 = actor2.pos.x + 1;
    let facing2 = get_facing(actor2.dir);
    let r2 = 1000 * row2 + 4 * column2 + facing2;
    println!("{}", r2);
}
