use std::{fmt, io};

#[derive(Clone, Copy)]
struct Block {
    n: u32,
    id: usize,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.n {
            write!(f, "{}", self.id)?;
        }
        Ok(())
    }
}

fn compact(disk_map: &[Block]) -> Vec<Block> {
    let mut disk_map = Vec::from(disk_map);

    let mut r = vec![];
    let mut left = 0;
    let mut right = disk_map.len() - 1;
    while left < disk_map.len() && left <= right {
        let b_left = disk_map[left];
        r.push(b_left);
        left += 1;

        if left > right {
            break;
        }

        let mut n_spaces = disk_map[left].n;
        let mut b_right = &mut disk_map[right];
        while n_spaces > b_right.n {
            r.push(*b_right);
            n_spaces -= b_right.n;
            right -= 2;
            b_right = &mut disk_map[right];
        }
        let b = Block {
            n: n_spaces,
            id: b_right.id,
        };
        r.push(b);
        b_right.n -= n_spaces;
        if b_right.n == 0 {
            right -= 2;
        }
        left += 1;
    }
    r
}

fn compact2(disk_map: &[Block]) -> Vec<Block> {
    let mut disk_map = Vec::from(disk_map);
    for right in (1..disk_map.len()).rev() {
        let b_right = disk_map[right];
        if b_right.id == 0 {
            continue;
        }
        for left in 1..right {
            let b_left = disk_map[left];
            if b_left.id != 0 {
                continue;
            }
            if b_left.n < b_right.n {
                continue;
            }
            if b_left.n == b_right.n {
                disk_map[left].id = b_right.id;
                disk_map[right].id = 0;
                break;
            }
            if b_left.n > b_right.n {
                disk_map[left].n -= b_right.n;
                disk_map[right].id = 0;
                disk_map.insert(left, b_right);
                break;
            }
        }
    }
    disk_map
}

fn checksum(blocks: Vec<Block>) -> usize {
    let mut i = 0;
    let mut r = 0;
    for b in blocks {
        for _ in 0..b.n {
            r += i * b.id;
            i += 1;
        }
    }
    r
}

fn main() {
    let mut input = String::new();
    let r = io::stdin().read_line(&mut input);
    if r.is_err() {
        panic!("Invalid input");
    }

    let disk_map = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .enumerate()
        .map(|(i, n)| {
            if i % 2 == 0 {
                Block { n, id: i / 2 }
            } else {
                Block { n, id: 0 }
            }
        })
        .collect::<Vec<_>>();

    let compact_blocks = compact(&disk_map);
    let r1 = checksum(compact_blocks);
    println!("{}", r1);

    let compact_blocks2 = compact2(&disk_map);
    let r2 = checksum(compact_blocks2);
    println!("{}", r2);
}
