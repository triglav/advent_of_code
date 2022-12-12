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

fn map_moves(
    grid: &Grid,
    width: usize,
    height: usize,
    start: Coord,
    reverse: bool,
) -> HashMap<Coord, u32> {
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
            if !moves.contains_key(&n) {
                match reverse {
                    false => {
                        if ne <= e + 1 {
                            moves.insert(n, v + 1);
                            todo.push_back(n);
                        }
                    }
                    true => {
                        if e <= ne || e - 1 == ne {
                            moves.insert(n, v + 1);
                            todo.push_back(n);
                        }
                    }
                }
            }
        }
    }
    moves
}

fn find_lowest(grid: &Grid, width: usize, height: usize) -> Vec<Coord> {
    let mut v = vec![];
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 0 {
                v.push(Coord { x, y });
            }
        }
    }
    v
}

fn main() {
    let (grid, start, end) = read_grid();
    let height = grid.len();
    let width = grid[0].len();

    let moves1 = map_moves(&grid, width, height, start, false);
    let r1 = moves1.get(&end).unwrap();
    println!("{}", r1);

    let moves2 = map_moves(&grid, width, height, end, true);
    let starts2 = find_lowest(&grid, width, height);
    let r2 = starts2
        .iter()
        .filter(|s| moves2.contains_key(s))
        .map(|s| moves2.get(s).unwrap())
        .min()
        .unwrap();
    println!("{}", r2);
}
