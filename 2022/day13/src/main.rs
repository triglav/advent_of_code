use std::{io, str::Chars};

enum Token {
    Integer(u8),
    ListStart,
    ListEnd,
}

fn read_integer(p: &Vec<char>, i: &mut usize) -> Option<Token> {
    let mut s = String::new();
    while *i < p.len() {
        let c = p.get(*i).unwrap();
        if !c.is_digit(10) {
            break;
        }
        s.push(*c);
        *i += 1;
    }
    assert!(s.len() > 0);
    Some(Token::Integer(s.parse().unwrap()))
}

fn read(p: &Vec<char>, i: &mut usize) -> Option<Token> {
    if *i >= p.len() {
        return None;
    }
    loop {
        let c = p.get(*i).unwrap();
        match c {
            ',' => {
                *i += 1;
                continue;
            }
            '[' => {
                *i += 1;
                break Some(Token::ListStart);
            }
            ']' => {
                *i += 1;
                break Some(Token::ListEnd);
            }
            '0'..='9' => break read_integer(p, i),
            _ => panic!("Unexpected character: '{}'", c),
        }
    }
}

fn read_tokens(s: Chars) -> Vec<Token> {
    let chars: Vec<_> = s.collect();
    let mut i: usize = 0;

    let mut v = vec![];
    loop {
        match read(&chars, &mut i) {
            Some(t) => v.push(t),
            None => break,
        }
    }
    v
}

fn packets_in_right_order(l: Chars, r: Chars) -> bool {
    let mut l = read_tokens(l);
    let mut r = read_tokens(r);

    let mut il: usize = 0;
    let mut ir: usize = 0;
    loop {
        let tl = l.get(il).unwrap();
        let tr = r.get(ir).unwrap();
        match tl {
            Token::Integer(vl) => {
                match tr {
                    Token::Integer(vr) => {
                        if vl < vr {
                            break true;
                        }
                        if vl > vr {
                            break false;
                        }
                        // continue
                    }
                    Token::ListStart => {
                        l.insert(il, Token::ListStart);
                        l.insert(il + 2, Token::ListEnd);
                        continue;
                    }
                    Token::ListEnd => break false,
                }
            }
            Token::ListStart => {
                match tr {
                    Token::Integer(_) => {
                        r.insert(ir, Token::ListStart);
                        r.insert(ir + 2, Token::ListEnd);
                        continue;
                    }
                    Token::ListStart => {
                        // continue
                    }
                    Token::ListEnd => break false,
                }
            }
            Token::ListEnd => {
                match tr {
                    Token::Integer(_) => break true,
                    Token::ListStart => break true,
                    Token::ListEnd => {
                        // continue
                    }
                }
            }
        };
        il += 1;
        ir += 1;
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let packet_lines: Vec<_> = lines.split(|l| l.is_empty()).collect();
    let r1: usize = packet_lines
        .iter()
        .map(|p| packets_in_right_order(p[0].chars(), p[1].chars()))
        .enumerate()
        .filter_map(|(idx, b)| if b { Some(idx + 1) } else { None })
        .sum();
    println!("{}", r1);
}
