use std::{
    collections::{HashMap, VecDeque},
    io,
};

fn parse_gate(s: &str) -> (String, u8) {
    let (name, value) = s.split_once(": ").unwrap();
    (name.to_string(), value.parse().unwrap())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ConnectionType {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
struct Connection {
    a: String,
    b: String,
    op: ConnectionType,
}

impl Connection {
    fn and(a: String, b: String) -> Connection {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            op: ConnectionType::And,
        }
    }

    fn xor(a: String, b: String) -> Connection {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            op: ConnectionType::Xor,
        }
    }

    fn or(a: String, b: String) -> Connection {
        Self {
            a: a.to_string(),
            b: b.to_string(),
            op: ConnectionType::Or,
        }
    }
}

fn simulate(gates: &HashMap<String, u8>, c: Connection) -> Option<u8> {
    if let Some(v1) = gates.get(&c.a) {
        if let Some(v2) = gates.get(&c.b) {
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

fn parse_connection(s: &str) -> (Connection, String) {
    let t = s.split_whitespace().collect::<Vec<_>>();
    let g1 = t[0].min(t[2]).to_string();
    let g2 = t[0].max(t[2]).to_string();
    let r = t[4];
    let op = t[1];
    let c = match op {
        "AND" => Connection::and(g1, g2),
        "XOR" => Connection::xor(g1, g2),
        "OR" => Connection::or(g1, g2),
        _ => panic!("Unknown operator: {}", op),
    };
    (c, r.to_string())
}

fn evaluate(
    gates: &HashMap<String, u8>,
    connections: &[(Connection, String)],
) -> HashMap<String, u8> {
    let mut gates = gates.clone();
    let mut todo = VecDeque::<(Connection, String)>::new();
    todo.extend(connections.to_owned());
    while let Some((c, r)) = todo.pop_front() {
        if let Some(v) = simulate(&gates, c.clone()) {
            gates.insert(r, v);
        } else {
            todo.push_back((c, r));
        }
    }
    gates
}

fn decode_gates(gates: &HashMap<String, u8>, gate_letter: char) -> u128 {
    let mut gates = gates
        .iter()
        .filter(|(n, _v)| n.starts_with(gate_letter))
        .collect::<Vec<_>>();
    gates.sort();
    gates
        .into_iter()
        .enumerate()
        .fold(0u128, |acc, (i, (_, v))| acc | ((*v as u128) << i))
}

fn swap_gate_outputs(
    connections: &[(Connection, String)],
    a: &str,
    b: &str,
) -> Vec<(Connection, String)> {
    let i1 = connections
        .iter()
        .enumerate()
        .find(|(_, c)| c.1 == a)
        .map(|(i, _)| i)
        .unwrap();
    let i2 = connections
        .iter()
        .enumerate()
        .find(|(_, c)| c.1 == b)
        .map(|(i, _)| i)
        .unwrap();

    let mut connections = Vec::from(connections);
    let r = connections[i1].1.clone();
    connections[i1].1 = connections[i2].1.clone();
    connections[i2].1 = r;
    connections
}

fn filter_connections<'a>(
    a: &str,
    b: &str,
    connections: &'a [(Connection, String)],
) -> Vec<&'a (Connection, String)> {
    let g1 = a.min(b);
    let g2 = a.max(b);
    connections
        .iter()
        .filter(|(c, _)| c.a == g1 && c.b == g2)
        .collect()
}

// A xor B -> S
// A and B -> C
fn check_half_adder(a: &str, b: &str, connections: &[(Connection, String)]) -> (String, String) {
    let c_ab = filter_connections(a, b, connections);
    assert_eq!(c_ab.len(), 2);
    let ab_s = c_ab
        .iter()
        .find(|(c, _)| c.op == ConnectionType::Xor)
        .unwrap()
        .1
        .clone();
    let c_out = c_ab
        .iter()
        .find(|(c, _)| c.op == ConnectionType::And)
        .unwrap()
        .1
        .clone();
    (ab_s, c_out)
}

// A xor B -> a_xor_b
// A and B -> a_and_b
// a_xor_b xor C_i -> S
// a_xor_b and C_i -> AxBaCi
// AxBaCi or a_and_b -> C_o
fn check_full_adder(
    c_in: String,
    i: usize,
    connections: &[(Connection, String)],
) -> Result<(String, String), (String, String)> {
    let a = format!("x{:02}", i);
    let b = format!("y{:02}", i);

    let c_ab = filter_connections(&a, &b, connections);
    let a_xor_b = c_ab
        .iter()
        .find(|(c, _)| c.op == ConnectionType::Xor)
        .unwrap();
    assert!(!a_xor_b.1.starts_with("z"));

    let a_and_b = c_ab
        .iter()
        .find(|(c, _)| c.op == ConnectionType::And)
        .unwrap();

    let c_abc = filter_connections(&a_xor_b.1, &c_in, connections);
    if c_abc.is_empty() {
        return Err((a_xor_b.1.to_owned(), a_and_b.1.to_owned()));
    }
    let ab_s = c_abc
        .iter()
        .find(|(c, _)| c.op == ConnectionType::Xor)
        .unwrap();
    if a_and_b.1.starts_with("z") && !ab_s.1.starts_with("z") {
        return Err((ab_s.1.to_owned(), a_and_b.1.to_owned()));
    }

    assert!(!a_and_b.1.starts_with("z"), "starts with 'z' {}", a_and_b.1);

    let a_xor_b_and_c = c_abc
        .iter()
        .find(|(c, _)| c.op == ConnectionType::And)
        .unwrap();

    if !ab_s.1.starts_with("z") && a_xor_b_and_c.1.starts_with("z") {
        return Err((ab_s.1.to_owned(), a_xor_b_and_c.1.to_owned()));
    }
    assert!(
        !a_xor_b_and_c.1.starts_with("z"),
        "{:?} starts with 'z'",
        a_xor_b_and_c.1
    );

    let c_axbc = filter_connections(&a_xor_b_and_c.1, &a_and_b.1, connections);
    let c_out = c_axbc
        .iter()
        .find(|(c, _)| c.op == ConnectionType::Or)
        .unwrap();

    if !ab_s.1.starts_with("z") && c_out.1.starts_with("z") {
        return Err((ab_s.1.to_owned(), c_out.1.to_owned()));
    }
    assert!(!c_out.1.starts_with("z"), "{:?} starts with 'z'", c_out.1);
    assert!(
        ab_s.1.starts_with("z"),
        "{:?} does not start with 'z'",
        ab_s.1
    );
    Ok((ab_s.1.clone(), c_out.1.clone()))
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

    let gates1 = evaluate(gates, connections);
    let r1 = decode_gates(&gates1, 'z');
    println!("{}", r1);

    let x = decode_gates(gates, 'x');
    let y = decode_gates(gates, 'y');
    let expected = x + y;

    let len = gates.iter().filter(|(n, _v)| n.starts_with('x')).count();

    let (_s00, c00) = check_half_adder("x00", "y00", connections);
    let mut carry = c00;
    let mut to_swap = Vec::new();
    let mut connections42 = connections.clone();
    for i in 1..len - 1 {
        match check_full_adder(carry.clone(), i, connections) {
            Ok((_s, c)) => {
                carry = c;
            }
            Err((g1, g2)) => {
                let connections2 = swap_gate_outputs(connections, &g1, &g2);
                connections42 = swap_gate_outputs(&connections42, &g1, &g2);
                to_swap.push(g1);
                to_swap.push(g2);
                let (_s, c) = check_full_adder(carry.clone(), i, &connections2).unwrap();
                carry = c;
            }
        }
    }
    let gates = evaluate(gates, &connections42);
    let fixed = decode_gates(&gates, 'z');
    assert_eq!(fixed.abs_diff(expected), 0);

    to_swap.sort();
    let r2 = to_swap.join(",");
    println!("{}", r2);
}
