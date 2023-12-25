use core::fmt;
use std::{cmp::Ordering, collections::HashMap, io};

use itertools::{iproduct, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z.cmp(&other.z) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.y.cmp(&other.y) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.x.cmp(&other.x)
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    id: usize,
    start: Coord,
    end: Coord,
}

impl Block {
    fn new(id: usize, start: Coord, end: Coord) -> Block {
        Block {
            id,
            start: Coord {
                x: start.x.min(end.x),
                y: start.y.min(end.y),
                z: start.z.min(end.z) - 1,
            },
            end: Coord {
                x: start.x.max(end.x),
                y: start.y.max(end.y),
                z: start.z.max(end.z) - 1,
            },
        }
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.end.cmp(&other.end)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}->{}", self.id, self.start, self.end)
    }
}

fn parse(id: usize, line: &str) -> Block {
    fn parse_coord(t: &str) -> Coord {
        let t = t
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Coord {
            x: t[0],
            y: t[1],
            z: t[2],
        }
    }
    let t = line.split('~').collect::<Vec<&str>>();
    Block::new(id, parse_coord(t[0]), parse_coord(t[1]))
}

struct World {
    tiles: Vec<[usize; 10 * 10]>,
    blocks: HashMap<usize, Block>,
}

impl World {
    fn new() -> World {
        World {
            tiles: vec![[0; 10 * 10]; 10 * 10],
            blocks: HashMap::new(),
        }
    }

    fn set(&mut self, x: usize, y: usize, z: usize, id: usize) {
        assert!(x < 10 && y < 10);
        if z + 1 > self.tiles.len() {
            self.tiles.resize(z + 1, [0; 10 * 10]);
        }
        self.tiles[z][y * 10 + x] = id;
    }

    fn get(&self, x: usize, y: usize, z: usize) -> usize {
        assert!(x < 10 && y < 10);
        assert!(z < self.tiles.len());
        self.tiles[z][y * 10 + x]
    }

    fn add_block(&mut self, block: Block) {
        iproduct!(
            block.start.x..=block.end.x,
            block.start.y..=block.end.y,
            block.start.z..=block.end.z
        )
        .for_each(|(x, y, z)| self.set(x, y, z, block.id));
        self.blocks.insert(block.id, block);
    }

    fn drop_block(&mut self, id: usize) {
        let b = *self.blocks.get(&id).unwrap();
        assert!(b.start.z > 0);
        iproduct!(b.start.x..=b.end.x, b.start.y..=b.end.y).for_each(|(x, y)| {
            self.set(x, y, b.end.z, 0);
            self.set(x, y, b.start.z - 1, b.id);
        });
        let b = self.blocks.get_mut(&id).unwrap();
        b.start.z -= 1;
        b.end.z -= 1;
    }

    fn is_free_below(&self, id: usize) -> bool {
        let b = self.blocks.get(&id).unwrap();
        if b.start.z == 0 {
            return false;
        }
        iproduct!(b.start.x..=b.end.x, b.start.y..=b.end.y)
            .all(|(x, y)| self.get(x, y, b.start.z - 1) == 0)
    }

    fn drop_blocks(&mut self) {
        let mut blocks = self.blocks.clone().into_values().collect_vec();
        blocks.sort();
        blocks.iter().map(|b| b.id).for_each(|id| loop {
            if !self.is_free_below(id) {
                break;
            }
            self.drop_block(id);
        });
    }

    fn get_blocks_below(&self, b: &Block) -> Vec<usize> {
        iproduct!(b.start.x..=b.end.x, b.start.y..=b.end.y)
            .map(|(x, y)| self.get(x, y, b.start.z - 1))
            .filter(|&id| id != 0)
            .unique()
            .collect::<Vec<usize>>()
    }

    fn get_blocks_above(&self, b: &Block) -> Vec<usize> {
        iproduct!(b.start.x..=b.end.x, b.start.y..=b.end.y)
            .map(|(x, y)| self.get(x, y, b.end.z + 1))
            .filter(|&id| id != 0)
            .unique()
            .collect::<Vec<usize>>()
    }

    fn analyse(&self) -> usize {
        let can_disintegrate = self
            .blocks
            .iter()
            .filter_map(|(id, b)| {
                let above = self.get_blocks_above(b);
                if above.is_empty() {
                    return Some(id);
                }
                let all_above_have_other_support = above.into_iter().all(|id2| {
                    let b2 = self.blocks.get(&id2).unwrap();
                    let number_of_other_supports = self
                        .get_blocks_below(b2)
                        .into_iter()
                        .filter(|&b3| b3 != *id)
                        .count();
                    number_of_other_supports > 0
                });
                if all_above_have_other_support {
                    Some(id)
                } else {
                    None
                }
            })
            .count();
        can_disintegrate
    }
}

fn main() {
    let blocks = io::stdin()
        .lines()
        .enumerate()
        .map(|(i, l)| parse(i + 1, l.unwrap().as_str()))
        .collect::<Vec<Block>>();

    let mut world = World::new();
    blocks.iter().for_each(|b| world.add_block(*b));
    world.drop_blocks();

    let r1 = world.analyse();
    println!("{}", r1);
}
