use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
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
}

fn parse_elves(lines: &Vec<String>) -> HashSet<Coord> {
    let mut elves = HashSet::<Coord>::new();
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert(Coord::new(x as i32, y as i32));
            }
        }
    }
    elves
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn are_other_elves_around(elves: &HashSet<Coord>, pos: &Coord) -> bool {
    for y in -1..=1 {
        for x in -1..=1 {
            if y == 0 && x == 0 {
                continue;
            }
            let c = Coord {
                x: pos.x + x,
                y: pos.y + y,
            };
            if elves.contains(&c) {
                return true;
            }
        }
    }
    false
}

fn check_direction(elves: &HashSet<Coord>, pos: &Coord, dir: &Direction) -> bool {
    match *dir {
        Direction::North => {
            !elves.contains(&Coord {
                x: pos.x,
                y: pos.y - 1,
            }) && !elves.contains(&Coord {
                x: pos.x - 1,
                y: pos.y - 1,
            }) && !elves.contains(&Coord {
                x: pos.x + 1,
                y: pos.y - 1,
            })
        }
        Direction::South => {
            !elves.contains(&Coord {
                x: pos.x,
                y: pos.y + 1,
            }) && !elves.contains(&Coord {
                x: pos.x - 1,
                y: pos.y + 1,
            }) && !elves.contains(&Coord {
                x: pos.x + 1,
                y: pos.y + 1,
            })
        }
        Direction::West => {
            !elves.contains(&Coord {
                x: pos.x - 1,
                y: pos.y,
            }) && !elves.contains(&Coord {
                x: pos.x - 1,
                y: pos.y + 1,
            }) && !elves.contains(&Coord {
                x: pos.x - 1,
                y: pos.y - 1,
            })
        }
        Direction::East => {
            !elves.contains(&Coord {
                x: pos.x + 1,
                y: pos.y,
            }) && !elves.contains(&Coord {
                x: pos.x + 1,
                y: pos.y + 1,
            }) && !elves.contains(&Coord {
                x: pos.x + 1,
                y: pos.y - 1,
            })
        }
    }
}

fn add_direction(pos: &Coord, dir: &Direction) -> Coord {
    match *dir {
        Direction::North => Coord {
            x: pos.x,
            y: pos.y - 1,
        },
        Direction::South => Coord {
            x: pos.x,
            y: pos.y + 1,
        },
        Direction::West => Coord {
            x: pos.x - 1,
            y: pos.y,
        },
        Direction::East => Coord {
            x: pos.x + 1,
            y: pos.y,
        },
    }
}

fn propose_moves(
    elves: &HashSet<Coord>,
    directions: &Vec<Direction>,
) -> HashMap<Coord, HashSet<Coord>> {
    let mut moves = HashMap::<Coord, HashSet<Coord>>::new();
    for e in elves {
        let mut moved = false;
        if are_other_elves_around(elves, e) {
            for d in directions {
                if check_direction(elves, e, d) {
                    let p2 = add_direction(e, d);
                    if !moves.contains_key(&p2) {
                        moves.insert(p2, HashSet::<Coord>::new());
                    }
                    moves.get_mut(&p2).unwrap().insert(*e);
                    moved = true;
                    break;
                }
            }
        }
        if !moved {
            let mut o = HashSet::<Coord>::new();
            o.insert(*e);
            moves.insert(*e, o);
        }
    }
    moves
}

fn apply_moves(proposed_moves: &HashMap<Coord, HashSet<Coord>>) -> HashSet<Coord> {
    let mut elves = HashSet::<Coord>::new();
    for (d, origins) in proposed_moves {
        if origins.len() > 1 {
            for o in origins {
                elves.insert(*o);
            }
            continue;
        }
        elves.insert(*d);
    }
    elves
}

fn find_smallest_rect(elves: &HashSet<Coord>) -> (Coord, Coord) {
    let tl = elves
        .iter()
        .copied()
        .reduce(|a, b| Coord {
            x: min(a.x, b.x),
            y: min(a.y, b.y),
        })
        .unwrap();
    let br = elves
        .iter()
        .copied()
        .reduce(|a, b| Coord {
            x: max(a.x, b.x),
            y: max(a.y, b.y),
        })
        .unwrap();
    (tl, br)
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let mut elves = parse_elves(&lines);
    let mut directions: Vec<_> = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    for _ in 0..10 {
        let count = elves.len();
        let moves = propose_moves(&elves, &directions);
        elves = apply_moves(&moves);
        assert_eq!(count, elves.len());
        directions.rotate_left(1);
    }
    let (tl, br) = find_smallest_rect(&elves);
    let w = br.x - tl.x + 1;
    let h = br.y - tl.y + 1;
    let r1 = w * h - elves.len() as i32;
    println!("{}", r1);
}
