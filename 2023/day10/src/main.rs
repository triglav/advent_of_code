use std::{collections::VecDeque, io};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<char>,

    distances: Vec<Option<usize>>,
}

impl Grid {
    pub fn new(width: usize, height: usize, tiles: Vec<char>) -> Self {
        Self {
            width,
            height,
            tiles,
            distances: vec![None; width * height],
        }
    }

    fn idx_to_coord(&self, idx: usize) -> Coord {
        assert!(idx < self.tiles.len());
        let x = idx % self.width;
        let y = idx / self.width;
        Coord::new(x, y)
    }

    fn coord_to_idx(&self, c: &Coord) -> usize {
        assert!(c.x < self.width);
        assert!(c.y < self.height);
        c.y * self.width + c.x
    }

    pub fn find_start(&self) -> Coord {
        let idx = self.tiles.iter().position(|&c| c == 'S').unwrap();
        self.idx_to_coord(idx)
    }

    fn get_tile(&self, c: &Coord) -> char {
        self.tiles[self.coord_to_idx(c)]
    }

    fn north(&self, c: &Coord) -> Option<Coord> {
        if c.y == 0 {
            None
        } else {
            Some(Coord::new(c.x, c.y - 1))
        }
    }

    fn south(&self, c: &Coord) -> Option<Coord> {
        if c.y >= self.height - 1 {
            None
        } else {
            Some(Coord::new(c.x, c.y + 1))
        }
    }

    fn east(&self, c: &Coord) -> Option<Coord> {
        if c.x >= self.width - 1 {
            None
        } else {
            Some(Coord::new(c.x + 1, c.y))
        }
    }

    fn west(&self, c: &Coord) -> Option<Coord> {
        if c.x == 0 {
            None
        } else {
            Some(Coord::new(c.x - 1, c.y))
        }
    }

    fn find_start_neighbours(&self, s: &Coord) -> Vec<Option<Coord>> {
        let neighbours = vec![self.north(s), self.south(s), self.east(s), self.west(s)]
            .into_iter()
            .flatten()
            .map(|c| (c, self.get_neighbours(&c)))
            .filter(|(_, n)| n.iter().any(|c| c == s))
            .map(|(c, _)| c)
            .collect::<Vec<_>>();
        assert_eq!(neighbours.len(), 2);
        vec![Some(neighbours[0]), Some(neighbours[1])]
    }

    pub fn get_neighbours(&self, c: &Coord) -> Vec<Coord> {
        let tile = self.get_tile(c);
        let coords = match tile {
            '|' => vec![self.north(c), self.south(c)],
            '-' => vec![self.east(c), self.west(c)],
            'L' => vec![self.north(c), self.east(c)],
            'J' => vec![self.north(c), self.west(c)],
            '7' => vec![self.south(c), self.west(c)],
            'F' => vec![self.south(c), self.east(c)],
            '.' => vec![],
            'S' => self.find_start_neighbours(c),
            _ => panic!("Unknown tile: {}", tile),
        };
        coords.into_iter().flatten().collect()
    }

    pub fn fill_distances(&mut self, start: Coord) {
        let mut todo = VecDeque::from([(start, 0usize)]);
        while let Some((c0, s0)) = todo.pop_front() {
            let idx = self.coord_to_idx(&c0);
            let s = self.distances[idx];
            if s.is_some() && s.unwrap() <= s0 {
                continue;
            }
            self.distances[idx] = Some(s0);
            self.get_neighbours(&c0)
                .into_iter()
                .for_each(|c| todo.push_back((c, s0 + 1)));
        }
    }

    pub fn find_largest_distance(&self) -> usize {
        self.distances
            .iter()
            .filter_map(|&d| d)
            .max()
            .expect("No distance found")
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let width = lines[0].len();
    let height = lines.len();
    let tiles = lines
        .into_iter()
        .flat_map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut grid = Grid::new(width, height, tiles);
    let start = grid.find_start();
    grid.fill_distances(start);

    let r1 = grid.find_largest_distance();
    println!("{:?}", r1);
}
