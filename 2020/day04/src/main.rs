use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use regex::Regex;

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
    fn check_passport2(p: &String) -> bool {
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

        let re_hgt = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        let re_hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let re_pid = Regex::new(r"^[0-9]{9}$").unwrap();

        p.split(' ').for_each(|t| {
            let i: Vec<&str> = t.split(':').collect();
            let key = i[0];
            *test.get_mut(key).unwrap() = match key {
                "byr" => {
                    let byr = i[1].parse::<u32>();
                    match byr {
                        Ok(v) => v >= 1920 && v <= 2002,
                        _ => false,
                    }
                }
                "iyr" => {
                    let iyr = i[1].parse::<u32>();
                    match iyr {
                        Ok(v) => v >= 2010 && v <= 2020,
                        _ => false,
                    }
                }
                "eyr" => {
                    let eyr = i[1].parse::<u32>();
                    match eyr {
                        Ok(v) => v >= 2020 && v <= 2030,
                        _ => false,
                    }
                }
                "hgt" => {
                    let m = re_hgt.captures(i[1]);
                    match m {
                        Some(m) => match (m.get(1), m.get(2)) {
                            (Some(v), Some(u)) => match v.as_str().parse::<u32>() {
                                Ok(n) => match u.as_str() {
                                    "cm" => n >= 150 && n <= 193,
                                    "in" => n >= 59 && n <= 76,
                                    _ => false,
                                },
                                _ => false,
                            },
                            _ => false,
                        },
                        _ => false,
                    }
                }
                "hcl" => re_hcl.is_match(i[1]),
                "ecl" => match i[1] {
                    "amb" => true,
                    "blu" => true,
                    "brn" => true,
                    "gry" => true,
                    "grn" => true,
                    "hzl" => true,
                    "oth" => true,
                    _ => false,
                },
                "pid" => re_pid.is_match(i[1]),
                "cid" => true,
                _ => false,
            };
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
    let r = v
        .iter()
        .map(|p| check_passport2(p))
        .filter(|v| *v == true)
        .count();
    println!("{}", r);
}
