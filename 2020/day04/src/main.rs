use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let mut v: Vec<String> = vec!["".to_owned()];
    for l in lines.iter() {
        if l.is_empty() {
            v.push("".to_owned());
            continue;
        }
        let s = v.last_mut().unwrap();
        if !s.is_empty() {
            s.push_str(" ");
        }
        s.push_str(l);
    }
    let v = v;

    fn check_passport(p: &String) -> bool {
        let mut test = HashMap::from([
            ("byr", false),
            ("iyr", false),
            ("eyr", false),
            ("hgt", false),
            ("hcl", false),
            ("ecl", false),
            ("pid", false),
            ("cid", false),
        ]);

        p.split(' ').for_each(|t| {
            let i: Vec<&str> = t.split(':').collect();
            let key = i[0];
            *test.get_mut(key).unwrap() = true;
        });

        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .into_iter()
            .fold(true, |a, i| a && *test.get(i).unwrap())
    }
    let r = v
        .iter()
        .map(|p| check_passport(p))
        .filter(|v| *v == true)
        .count();
    println!("{}", r);
}
