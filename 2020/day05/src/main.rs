use itertools::Itertools;
use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    fn find((l, u): (u32, u32), input: &[u8], (c_l, c_u): (char, char)) -> u32 {
        if l == u {
            return l;
        }
        match input[0] as char {
            c if c == c_l => find((l, l + (u - l) / 2), &input[1..], (c_l, c_u)),
            c if c == c_u => find((l + (u - l) / 2 + 1, u), &input[1..], (c_l, c_u)),
            _ => panic!("invalid input"),
        }
    }

    fn find_row(input: &[u8]) -> u32 {
        find((0, 127), input, ('F', 'B'))
    }

    fn find_column(input: &[u8]) -> u32 {
        find((0, 7), input, ('L', 'R'))
    }

    let ids: HashSet<_> = lines
        .iter()
        .map(|l| {
            let b = l.as_bytes();
            let row = find_row(b);
            let column = find_column(&b[7..]);
            row * 8 + column
        })
        .collect();
    let r = ids.iter().max();
    println!("{}", r.unwrap());

    let available_seats: HashSet<_> = (9..120)
        .cartesian_product(0..8)
        .map(|(r, c)| r * 8 + c)
        .collect();
    let free_seats: Vec<_> = available_seats.difference(&ids).sorted().collect();
    assert!(free_seats.len() == 1);
    println!("{}", free_seats[0]);
}
