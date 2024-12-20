use std::{fmt, io, ops};

use itertools::{iproduct, Itertools};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Coords {
    x: i64,
    y: i64,
}

impl Coords {
    pub fn north() -> Self {
        Coords { x: 0, y: -1 }
    }
    pub fn south() -> Self {
        Coords { x: 0, y: 1 }
    }
    pub fn west() -> Self {
        Coords { x: -1, y: 0 }
    }
    pub fn east() -> Self {
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

#[derive(Clone)]
struct Grid<T> {
    pub width: i64,
    pub height: i64,
    pub tiles: Vec<T>,
}

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display + Copy + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        iproduct!(0..self.height, 0..self.width).try_for_each(|(y, x)| {
            let t = self.tiles[(y * self.width + x) as usize];
            write!(f, "{}", t)?;
            if x == self.width - 1 {
                writeln!(f)?;
            }
            Ok(())
        })
    }
}

impl<T> Grid<T>
where
    T: Copy + Default + PartialEq,
{
    pub fn from(grid: Vec<Vec<T>>) -> Grid<T> {
        let width = grid[0].len() as i64;
        let height = grid.len() as i64;
        let tiles = grid.into_iter().flatten().collect::<Vec<_>>();
        Grid {
            width,
            height,
            tiles,
        }
    }

    pub fn get(&self, p: Coords) -> T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles[(p.y * self.width + p.x) as usize]
    }

    pub fn get_mut(&mut self, p: Coords) -> &mut T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        self.tiles
            .get_mut((p.y * self.width + p.x) as usize)
            .unwrap()
    }

    pub fn find(&self, t: T) -> Option<Coords> {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, &x)| x == t)
            .map(|(i, _)| Coords {
                x: i as i64 % self.width,
                y: i as i64 / self.width,
            })
    }

    pub fn get_neightbours(&self, x: i64, y: i64) -> Vec<Coords> {
        vec![
            Coords { x: x - 1, y },
            Coords { x: x + 1, y },
            Coords { x, y: y - 1 },
            Coords { x, y: y + 1 },
        ]
        .into_iter()
        .filter(|c| c.x >= 0 && c.x < self.width && c.y >= 0 && c.y < self.height)
        .collect()
    }
}

fn evaluate_track(grid: &Grid<char>, start: Coords, end: Coords) -> Grid<Option<usize>> {
    let mut r = Grid::<Option<usize>> {
        width: grid.width,
        height: grid.height,
        tiles: vec![None; (grid.width * grid.height) as usize],
    };
    let mut d = 0;
    let mut p = (start, start);
    loop {
        assert_ne!(grid.get(p.0), '#');
        *r.get_mut(p.0) = Some(d);
        if p.0 == end {
            break;
        }

        let next = grid
            .get_neightbours(p.0.x, p.0.y)
            .into_iter()
            .filter(|&n| n != p.1 && grid.get(n) != '#')
            .collect::<Vec<_>>();
        assert_eq!(next.len(), 1);
        p = (next[0], p.0);
        d += 1;
    }
    r
}

fn get_cheat_neighbours(track: &Grid<Option<usize>>, c: Coords) -> Vec<(usize, usize)> {
    assert_eq!(track.get(c), None);
    let mut r = vec![];
    if let Some(n) = track.get(c + Coords::north()) {
        if let Some(s) = track.get(c + Coords::south()) {
            r.push((n, s));
        }
    }
    if let Some(e) = track.get(c + Coords::east()) {
        if let Some(w) = track.get(c + Coords::west()) {
            r.push((e, w));
        }
    }
    r
}

fn find_cheats(track: &Grid<Option<usize>>) -> Vec<usize> {
    iproduct!(1..track.height - 1, 1..track.width - 1)
        .map(|(y, x)| Coords { x, y })
        .filter(|&c| track.get(c).is_none())
        .flat_map(|c| get_cheat_neighbours(track, c))
        .map(|(p1, p2)| p1.abs_diff(p2) - 2)
        .collect_vec()
}

fn main() {
    let grid = Grid::from(
        io::stdin()
            .lines()
            .map(|l| l.unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let track = evaluate_track(&grid, start, end);
    let cheats = find_cheats(&track);

    let r1 = cheats.iter().filter(|&&c| c >= 100).count();
    println!("{}", r1);
}
