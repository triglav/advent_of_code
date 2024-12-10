use std::{fmt, io};

struct Block {
    n: u32,
    id: usize,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.n, self.id)
    }
}

fn compact(disk_map: Vec<u32>) -> Vec<Block> {
    let mut disk_map = disk_map.clone();

    let mut r = vec![];
    let mut left = 0;
    let mut left_id = 0;
    let mut right = disk_map.len() - 1;
    let mut right_id = disk_map.len() / 2;
    while left < disk_map.len() && left <= right {
        let b = Block {
            n: disk_map[left],
            id: left_id,
        };
        r.push(b);
        left += 1;
        left_id += 1;

        if left > right {
            break;
        }

        let mut n_spaces = disk_map[left];
        let mut n_right = disk_map[right];

        while n_spaces > n_right {
            let b = Block {
                n: disk_map[right],
                id: right_id,
            };
            r.push(b);
            n_spaces -= n_right;
            right -= 2;
            right_id -= 1;
            n_right = disk_map[right];
        }
        let b = Block {
            n: n_spaces,
            id: right_id,
        };
        r.push(b);
        disk_map[right] -= n_spaces;
        if disk_map[right] == 0 {
            right -= 2;
            right_id -= 1;
        }
        left += 1;
    }
    r
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
        .collect::<Vec<_>>();

    let compact_blocks = compact(disk_map);
    let r1 = checksum(compact_blocks);
    println!("{}", r1);
}
