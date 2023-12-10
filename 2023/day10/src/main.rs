use std::{collections::VecDeque, io};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Unknown = 0,
    Loop,
    Left,
    Right,
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

    fn new_coord(&self, x: i32, y: i32) -> Option<Coord> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            None
        } else {
            Some(Coord::new(x as usize, y as usize))
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

    fn get_neighbour_tiles(&self, c: &Coord) -> Vec<Coord> {
        vec![self.north(c), self.south(c), self.west(c), self.east(c)]
            .into_iter()
            .flatten()
            .collect()
    }

    pub fn fill_distances(&mut self, start: &Coord) {
        let mut todo = VecDeque::from([(*start, 0usize)]);
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

    fn get_left_right_tiles(&self, p: &Coord, c: &Coord) -> (Vec<Coord>, Vec<Coord>) {
        let px = p.x as i32;
        let py = p.y as i32;
        let cx = c.x as i32;
        let cy = c.y as i32;
        let dx = cx - px;
        let dy = cy - py;
        assert!(dx.abs() <= 1);
        assert!(dy.abs() <= 1);

        let (left, right) = match (dx, dy) {
            (1, 0) => (
                vec![self.north(p), self.north(c)], // left
                vec![self.south(p), self.south(c)], // right
            ),
            (1, 1) => (
                vec![self.new_coord(px + 1, py)], // left
                vec![self.new_coord(px, py + 1)], // right
            ),
            (0, 1) => (
                vec![self.east(p), self.east(c)], // left
                vec![self.west(p), self.west(c)], // right
            ),
            (-1, 1) => (
                vec![self.new_coord(px, py + 1)], // left
                vec![self.new_coord(px - 1, py)], // right
            ),
            (-1, 0) => (
                vec![self.south(p), self.south(c)], // left
                vec![self.north(p), self.north(c)], // right
            ),
            (-1, -1) => (
                vec![self.new_coord(px - 1, py)], // left
                vec![self.new_coord(px, py - 1)], // right
            ),
            (0, -1) => (
                vec![self.west(p), self.west(c)], // left
                vec![self.east(p), self.east(c)], // right
            ),
            (1, -1) => (
                vec![self.new_coord(px, py - 1)], // left
                vec![self.new_coord(px + 1, py)], // right
            ),
            _ => panic!("Invalid loop"),
        };
        (
            left.into_iter().flatten().collect::<Vec<_>>(),
            right.into_iter().flatten().collect::<Vec<_>>(),
        )
    }

    fn walk_loop(&self, start: &Coord) -> (Vec<Coord>, Vec<Coord>) {
        let mut prev = *start;
        let mut next = self.get_neighbours(start)[0];

        let mut left = vec![];
        let mut right = vec![];
        while next != *start {
            let (l, r) = self.get_left_right_tiles(&prev, &next);
            left.extend(l);
            right.extend(r);

            let neighbours = self.get_neighbours(&next);
            assert_eq!(neighbours.len(), 2);
            let a = neighbours[0];
            let b = neighbours[1];
            if a == prev {
                prev = next;
                next = b;
            } else if b == prev {
                prev = next;
                next = a;
            } else {
                panic!("Invalid loop");
            }
        }
        (left, right)
    }

    pub fn find_enclosed_tile_count(&self, start: &Coord) -> usize {
        let mut hit_map = vec![TileType::Unknown; self.width * self.height];
        self.distances
            .iter()
            .enumerate()
            .filter(|(_, &d)| d.is_some())
            .for_each(|(idx, _)| {
                hit_map[idx] = TileType::Loop;
            });
        let (left, right) = self.walk_loop(start);
        let left = left.into_iter().map(|c| (c, TileType::Left));
        let right = right.into_iter().map(|c| (c, TileType::Right));
        let mut todo = VecDeque::from(left.chain(right).collect::<Vec<_>>());

        while let Some((c0, tile_type)) = todo.pop_front() {
            let idx = self.coord_to_idx(&c0);
            if hit_map[idx] != TileType::Unknown {
                continue;
            }
            hit_map[idx] = tile_type;

            self.get_neighbour_tiles(&c0)
                .into_iter()
                .for_each(|c| todo.push_back((c, tile_type)));
        }
        // (0..self.height).for_each(|y| {
        //     (0..self.width).for_each(|x| {
        //         let idx = self.coord_to_idx(&Coord::new(x, y));
        //         let hit = hit_map[idx];
        //         match hit {
        //             TileType::Unknown => print!("I"),
        //             TileType::Loop => print!("."),
        //             TileType::Left => print!("L"),
        //             TileType::Right => print!("R"),
        //         }
        //     });
        //     println!();
        // });

        let v = (0..self.height).flat_map(|y| [Coord::new(0, y), Coord::new(self.width - 1, y)]);
        let h = (0..self.width).flat_map(|x| [Coord::new(x, 0), Coord::new(x, self.height - 1)]);
        let boundary_indices = v
            .chain(h)
            .map(|c| self.coord_to_idx(&c))
            .collect::<Vec<_>>();
        let left_on_boundary = boundary_indices
            .iter()
            .any(|&idx| hit_map[idx] == TileType::Left);
        let right_on_boundary = boundary_indices
            .iter()
            .any(|&idx| hit_map[idx] == TileType::Right);
        assert_ne!(left_on_boundary, right_on_boundary);

        if left_on_boundary {
            hit_map.iter().filter(|&t| *t == TileType::Right).count()
        } else {
            hit_map.iter().filter(|&t| *t == TileType::Left).count()
        }
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
    grid.fill_distances(&start);

    let r1 = grid.find_largest_distance();
    println!("{:?}", r1);

    let r2 = grid.find_enclosed_tile_count(&start);
    println!("{:?}", r2);
}
