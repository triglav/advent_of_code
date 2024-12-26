use std::{
    collections::{HashMap, VecDeque},
    io,
};

fn parse_gate(s: &str) -> (&str, u8) {
    let (name, value) = s.split_once(": ").unwrap();
    (name, value.parse().unwrap())
}

#[derive(Debug, Copy, Clone)]
enum ConnectionType {
    And,
    Xor,
    Or,
}

#[derive(Debug, Copy, Clone)]
struct Connection<'a> {
    a: &'a str,
    b: &'a str,
    op: ConnectionType,
}

impl<'a> Connection<'a> {
    fn and(a: &'a str, b: &'a str) -> Connection<'a> {
        Self {
            a,
            b,
            op: ConnectionType::And,
        }
    }

    fn xor(a: &'a str, b: &'a str) -> Connection<'a> {
        Self {
            a,
            b,
            op: ConnectionType::Xor,
        }
    }

    fn or(a: &'a str, b: &'a str) -> Connection<'a> {
        Self {
            a,
            b,
            op: ConnectionType::Or,
        }
    }
}

fn simulate(gates: &HashMap<&str, u8>, c: Connection) -> Option<u8> {
    if let Some(v1) = gates.get(c.a) {
        if let Some(v2) = gates.get(c.b) {
            let r = match c.op {
                ConnectionType::And => v1 & v2,
                ConnectionType::Xor => v1 ^ v2,
                ConnectionType::Or => v1 | v2,
            };
            return Some(r);
        }
    }
    None
}

fn parse_connection(s: &str) -> (Connection<'_>, &str) {
    let t = s.split_whitespace().collect::<Vec<_>>();
    let g1 = t[0];
    let g2 = t[2];
    let r = t[4];
    let op = t[1];
    let c = match op {
        "AND" => Connection::and(g1, g2),
        "XOR" => Connection::xor(g1, g2),
        "OR" => Connection::or(g1, g2),
        _ => panic!("Unknown operator: {}", op),
    };
    (c, r)
}

fn evaluate<'a>(
    gates: &HashMap<&'a str, u8>,
    connections: &[(Connection<'a>, &'a str)],
) -> HashMap<&'a str, u8> {
    let mut gates = gates.clone();
    let mut todo = VecDeque::<(Connection<'a>, &str)>::new();
    todo.extend(connections.iter());
    while let Some((c, r)) = todo.pop_front() {
        if let Some(v) = simulate(&gates, c) {
            gates.insert(r, v);
        } else {
            todo.push_back((c, r));
        }
    }
    gates
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let idx = lines
        .iter()
        .enumerate()
        .find(|(_, l)| l.is_empty())
        .map(|(i, _)| i)
        .unwrap();
    let gates = &lines[0..idx]
        .iter()
        .map(|l| parse_gate(l))
        .collect::<HashMap<_, _>>();
    let connections = &lines[idx + 1..]
        .iter()
        .map(|l| parse_connection(l))
        .collect::<Vec<_>>();

    let mut z = evaluate(gates, connections)
        .into_iter()
        .filter(|(n, _v)| n.starts_with('z'))
        .collect::<Vec<_>>();
    z.sort();
    let r1 = z
        .iter()
        .enumerate()
        .fold(0u128, |acc, (i, &(_, v))| acc | ((v as u128) << i));
    println!("{}", r1);
}
