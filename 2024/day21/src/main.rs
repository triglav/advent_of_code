use std::{collections::HashMap, fmt, io, ops};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    pub fn up() -> Self {
        Coords { x: 0, y: -1 }
    }
    pub fn down() -> Self {
        Coords { x: 0, y: 1 }
    }
    pub fn left() -> Self {
        Coords { x: -1, y: 0 }
    }
    pub fn right() -> Self {
        Coords { x: 1, y: 0 }
    }
}

impl fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coords {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Div for Coords {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::Mul for Coords {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<i64> for Coords {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Div<i64> for Coords {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

type Keypad = HashMap<char, Coords>;

fn make_numeric_keypad() -> Keypad {
    let mut keypad = Keypad::new();
    keypad.insert('7', Coords { x: 0, y: 0 });
    keypad.insert('8', Coords { x: 1, y: 0 });
    keypad.insert('9', Coords { x: 2, y: 0 });

    keypad.insert('4', Coords { x: 0, y: 1 });
    keypad.insert('5', Coords { x: 1, y: 1 });
    keypad.insert('6', Coords { x: 2, y: 1 });

    keypad.insert('1', Coords { x: 0, y: 2 });
    keypad.insert('2', Coords { x: 1, y: 2 });
    keypad.insert('3', Coords { x: 2, y: 2 });

    keypad.insert('X', Coords { x: 0, y: 3 });
    keypad.insert('0', Coords { x: 1, y: 3 });
    keypad.insert('A', Coords { x: 2, y: 3 });
    keypad
}

fn make_directional_keypad() -> Keypad {
    let mut keypad = Keypad::new();
    keypad.insert('X', Coords { x: 0, y: 0 });
    keypad.insert('^', Coords { x: 1, y: 0 });
    keypad.insert('A', Coords { x: 2, y: 0 });

    keypad.insert('<', Coords { x: 0, y: 1 });
    keypad.insert('v', Coords { x: 1, y: 1 });
    keypad.insert('>', Coords { x: 2, y: 1 });
    keypad
}

fn solve(keypads: &[&Keypad], password: &str) -> String {
    fn find_paths(keypad: &Keypad, from: Coords, to: Coords) -> Vec<String> {
        let pos_x = *keypad.get(&'X').unwrap();
        let mut r = vec![];
        let mut todo = vec![(from, "".to_string())];
        while let Some((p, path)) = todo.pop() {
            if p == to {
                r.push(path + "A");
                continue;
            }
            let d = to - p;
            if d.x != 0 {
                let dx = d.x / d.x.abs();
                if dx == -1 {
                    let p2 = p + Coords::left();
                    if p2 != pos_x {
                        todo.push((p2, path.clone() + "<"));
                    }
                } else if dx == 1 {
                    let p2 = p + Coords::right();
                    if p2 != pos_x {
                        todo.push((p2, path.clone() + ">"));
                    }
                }
            }
            if d.y != 0 {
                let dy = d.y / d.y.abs();
                if dy == -1 {
                    let p2 = p + Coords::up();
                    if p2 != pos_x {
                        todo.push((p2, path.clone() + "^"));
                    }
                } else if dy == 1 {
                    let p2 = p + Coords::down();
                    if p2 != pos_x {
                        todo.push((p2, path.clone() + "v"));
                    }
                }
            }
        }
        r
    }

    if keypads.is_empty() || password.is_empty() {
        return password.to_string();
    }
    let keypad = keypads[0];
    let mut p = keypad.get(&'A').unwrap();
    let mut sequence = "".to_string();
    for b in password.chars() {
        let p2 = keypad.get(&b).unwrap();
        let paths = find_paths(keypad, *p, *p2);
        let button_path = paths
            .into_iter()
            .map(|p| solve(&keypads[1..], &p))
            .reduce(|a, b| if a.len() < b.len() { a } else { b })
            .unwrap();
        sequence.push_str(&button_path);
        p = p2;
    }
    sequence
}

fn main() {
    let numeric_keypad = make_numeric_keypad();
    let directional_keypad = make_directional_keypad();

    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let r1 = lines
        .iter()
        .map(|password| {
            (
                password,
                solve(
                    &[&numeric_keypad, &directional_keypad, &directional_keypad],
                    password,
                ),
            )
        })
        .map(|(password, sequence)| {
            let numeric = password[0..password.len() - 1].parse::<usize>().unwrap();
            numeric * sequence.len()
        })
        .sum::<usize>();
    println!("{}", r1);
}
