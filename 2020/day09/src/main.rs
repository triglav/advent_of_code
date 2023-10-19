use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let preamble = 25;

    let r = (preamble..values.len()).find_map(|i| {
        let h = (i - preamble..i).fold(HashSet::new(), |mut a, c| {
            a.insert(values[c]);
            a
        });
        let x = values[i];
        let has_the_property = (1..=h.len()).find(|j| {
            let a = values[i - j];
            let b = x - a;
            h.contains(&b)
        });
        if has_the_property.is_some() {
            None
        } else {
            Some(x)
        }
    });
    println!("{}", r.unwrap());
}
