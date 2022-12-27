use core::panic;
use std::io;

fn convert_from_snafu_digit(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }
}

fn convert_from_snafu(snafu: &String) -> i64 {
    let dec = snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| 5_i64.pow(i as u32) * convert_from_snafu_digit(c))
        .sum::<i64>();
    dec
}

fn convert_to_snafu_number(dec: i64) -> (i64, i64) {
    match dec {
        0 => (0, 0),
        1 => (0, 1),
        2 => (0, 2),
        3 => (1, -2),
        4 => (1, -1),
        _ => panic!(),
    }
}

fn convert_to_snafu_digit(dec: i64) -> char {
    match dec {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!(),
    }
}

fn convert_to_snafu(mut dec: i64) -> String {
    let mut digits = vec![];
    for i in 0.. {
        let p0 = 5_i64.pow(i);
        let p = p0 * 5;
        let zz2 = dec % p;
        dec -= zz2;
        let zz3 = zz2 / p0;

        let (mut s1, s0) = convert_to_snafu_number(zz3);
        if let Some(d) = digits.get_mut(i as usize) {
            *d += s0;
            if *d > 2 {
                *d -= 5;
                s1 += 1;
            }
        } else {
            digits.push(s0);
        }
        assert_eq!(digits.len() - 1, i as usize);
        if s1 > 0 {
            digits.push(s1);
        }
        if dec == 0 {
            break;
        }
    }
    let snafu: String = digits
        .iter()
        .rev()
        .map(|d| convert_to_snafu_digit(*d))
        .collect();
    snafu
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();

    let dec = lines.iter().map(|l| convert_from_snafu(l)).sum::<i64>();
    let r1 = convert_to_snafu(dec);
    println!("{}", r1);
}
