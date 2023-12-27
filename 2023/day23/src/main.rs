use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    io,
};

use itertools::{iproduct, Itertools};

struct Grid<T> {
    pub width: i64,
    pub height: i64,
    tiles: Vec<T>,
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
    pub fn new(width: i64, height: i64, default: T) -> Grid<T> {
        let tiles = vec![default; (width * height) as usize];
        Grid {
            width,
            height,
            tiles,
        }
    }

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

    pub fn get(&self, x: i64, y: i64) -> T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn get_mut(&mut self, x: i64, y: i64) -> &mut T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.tiles.get_mut((y * self.width + x) as usize).unwrap()
    }

    pub fn get_neightbours(&self, x: i64, y: i64) -> Vec<(i64, i64)> {
        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .into_iter()
            .filter(|(x, y)| *x >= 0 && *x < self.width && *y >= 0 && *y < self.height)
            .collect()
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
struct Node {
    x: i64,
    y: i64,
}

impl Node {
    pub fn new(x: i64, y: i64) -> Node {
        Node { x, y }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Edge {
    to: Node,
    steps: usize,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  -> {} ({})", self.to, self.steps)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
enum EdgeDirection {
    Both,
    Forward,
    Backward,
    Blocked,
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<Node, Vec<Edge>>,
    end_y: i64,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.nodes.iter().try_for_each(|(n, e)| {
            writeln!(f, "{}", n)?;
            e.iter().try_for_each(|e| writeln!(f, "{}", e))?;
            Ok(())
        })
    }
}

impl Graph {
    pub fn trace(grid: &Grid<char>, start: Node) -> Graph {
        let mut nodes = HashMap::<Node, Vec<Edge>>::new();
        let mut hit_grid = Grid::new(grid.width, grid.height, false);

        let mut todo = Vec::new();
        todo.push((start, (start, 0usize, (0, 1), EdgeDirection::Both)));

        while let Some((p, (start, steps, dir, edge_dir))) = todo.pop() {
            if hit_grid.get(p.x, p.y) {
                continue;
            }

            if p.y == grid.height - 1 {
                assert_eq!(grid.get(p.x, p.y), '.');
                nodes.entry(start).or_default().push(Edge { to: p, steps });
                continue;
            }

            let n = grid
                .get_neightbours(p.x, p.y)
                .into_iter()
                .filter(|(x, y)| grid.get(*x, *y) != '#')
                .collect_vec();
            let t = grid.get(p.x, p.y);
            assert_ne!(t, '#');
            assert!(
                t == '.' || n.len() == 2,
                "Slope expected to have two neighbours! ({} {:?})",
                t,
                n
            );
            let is_slope = t != '.';
            let is_slope_same_dir = match t {
                '>' => dir.0 == 1 && dir.1 == 0,
                '<' => dir.0 == -1 && dir.1 == 0,
                '^' => dir.0 == 0 && dir.1 == -1,
                'v' => dir.0 == 0 && dir.1 == 1,
                '.' => true,
                _ => panic!(),
            };
            let new_edge_dir = match (is_slope, is_slope_same_dir, edge_dir) {
                (false, _, _) => edge_dir,
                (true, true, EdgeDirection::Both) => EdgeDirection::Forward,
                (true, true, EdgeDirection::Backward) => EdgeDirection::Blocked,
                (true, true, _) => edge_dir,
                (true, false, EdgeDirection::Both) => EdgeDirection::Backward,
                (true, false, EdgeDirection::Forward) => EdgeDirection::Blocked,
                (true, false, _) => edge_dir,
            };

            let is_crossing = n.len() > 2;
            *hit_grid.get_mut(p.x, p.y) = !is_crossing;

            if is_crossing {
                if new_edge_dir == EdgeDirection::Both || new_edge_dir == EdgeDirection::Forward {
                    nodes.entry(start).or_default().push(Edge { to: p, steps });
                }
                if new_edge_dir == EdgeDirection::Both || new_edge_dir == EdgeDirection::Backward {
                    nodes.entry(p).or_default().push(Edge { to: start, steps });
                }
            }
            let (start, steps, new_edge_dir) = if is_crossing {
                (p, 0, EdgeDirection::Both)
            } else {
                (start, steps, new_edge_dir)
            };

            n.into_iter()
                .filter(|(x, y)| !hit_grid.get(*x, *y))
                .for_each(|(x, y)| {
                    let d = (x - p.x, y - p.y);
                    if dir.0 + d.0 == 0 && dir.1 + d.1 == 0 {
                        return;
                    }
                    todo.push((Node::new(x, y), (start, steps + 1, d, new_edge_dir)));
                });
        }
        Graph {
            nodes,
            end_y: grid.height - 1,
        }
    }

    pub fn find_paths(&self, start: Node) -> Vec<usize> {
        let mut result = Vec::new();

        let mut todo = Vec::new();
        todo.push((start, 0usize, HashSet::new()));

        while let Some((n, steps, seen)) = todo.pop() {
            if n.y == self.end_y {
                result.push(steps);
                continue;
            }

            let edges = self.nodes.get(&n).unwrap();
            edges.iter().for_each(|e| {
                if seen.contains(&e.to) {
                    return;
                }
                let mut seen = seen.clone();
                seen.insert(e.to);
                todo.push((e.to, steps + e.steps, seen));
            });
        }
        result
    }
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect_vec())
        .collect_vec();
    let grid = Grid::from(lines);

    let start = Node::new(1, 0);
    let graph = Graph::trace(&grid, start);
    let r1 = graph.find_paths(start).into_iter().max().unwrap();
    println!("{}", r1);
}
