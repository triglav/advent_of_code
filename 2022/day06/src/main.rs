use itertools::{Itertools, MultiPeek};
use std::{io, str::Chars};

fn are_characters_distinct(iter: &mut MultiPeek<Chars>, size: usize) -> bool {
    let v = (0..size)
        .filter_map(|_| match iter.peek() {
            Some(n) => Some(*n),
            None => None,
        })
        .collect::<Vec<char>>();
    v.len() == size && v.iter().all_unique()
}

fn find_marker(iter: &mut MultiPeek<Chars>, size: usize) -> usize {
    let mut idx = 0;
    while let Some(_) = iter.next() {
        idx += 1;
        if are_characters_distinct(iter, size) {
            return idx + size;
        }
    }
    0
}

fn main() {
    let buf = io::stdin().lines().next().unwrap().unwrap();

    let mut i1 = buf.chars().multipeek();
    let r1 = find_marker(&mut i1, 4);
    println!("{}", r1);

    let mut i2 = buf.chars().multipeek();
    let r2 = find_marker(&mut i2, 14);
    println!("{}", r2);
}
