use std::{
    collections::{HashMap, VecDeque},
    io,
};

type Grid = Vec<Vec<u8>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

fn read_grid() -> (Grid, Coord, Coord) {
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };
    let mut grid = Grid::new();

    for (y, line) in io::stdin().lines().map(|l| l.unwrap()).enumerate() {
        let mut v = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start.x = x;
                    start.y = y;
                    v.push('a' as u8 - 'a' as u8);
                }
                'E' => {
                    end.x = x;
                    end.y = y;
                    v.push('z' as u8 - 'a' as u8);
                }
                _ => {
                    v.push(c as u8 - 'a' as u8);
                }
            }
        }
        grid.push(v);
    }
    (grid, start, end)
}

fn get_neighbours(c: Coord, width: usize, height: usize) -> Vec<Coord> {
    let mut v = vec![];
    if c.x > 0 {
        v.push(Coord { x: c.x - 1, y: c.y });
    }
    if c.x + 1 < width {
        v.push(Coord { x: c.x + 1, y: c.y });
    }
    if c.y > 0 {
        v.push(Coord { x: c.x, y: c.y - 1 });
    }
    if c.y + 1 < height {
        v.push(Coord { x: c.x, y: c.y + 1 });
    }
    v
}

fn main() {
    let (grid, start, end) = read_grid();
    let height = grid.len();
    let width = grid[0].len();

    let mut moves = HashMap::<Coord, u32>::new();
    moves.insert(start, 0);

    let mut todo = VecDeque::<Coord>::new();
    todo.push_back(start);

    while !todo.is_empty() {
        let c = todo.pop_front().unwrap();
        let e = grid[c.y][c.x];
        let v = *moves.get(&c).unwrap();

        for n in get_neighbours(c, width, height) {
            let ne = grid[n.y][n.x];
            if !moves.contains_key(&n) && ne <= e + 1 {
                moves.insert(n, v + 1);
                todo.push_back(n);
            }
        }
    }

    let r1 = moves.get(&end).unwrap();
    println!("{}", r1);
}
