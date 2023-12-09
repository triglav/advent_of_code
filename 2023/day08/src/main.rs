use std::{collections::HashMap, io};

fn parse(s: &str) -> (&str, (&str, &str)) {
    let mut t = s.split('=');
    let s = t.next().unwrap().trim();

    let t2 = t
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    let left = &t2[0][1..];
    let right = &t2[1][..3];
    (s, (left, right))
}

struct State {
    instruction_idx: usize,
    node: String,
    steps: usize,
}

fn process(s: &State, instructions: &str, nodes: &HashMap<&str, (&str, &str)>) -> State {
    let instruction = instructions.as_bytes()[s.instruction_idx] as char;
    let (left, right) = nodes.get(s.node.as_str()).unwrap();
    let instruction_idx = if s.instruction_idx + 1 >= instructions.len() {
        0
    } else {
        s.instruction_idx + 1
    };
    let node = match instruction {
        'L' => *left,
        'R' => *right,
        _ => panic!("Invalid instruction"),
    };
    State {
        instruction_idx,
        node: node.to_string(),
        steps: s.steps + 1,
    }
}

fn main() {
    let mut lines = io::stdin().lines();
    let instructions = lines.next().unwrap().unwrap();
    lines.next();
    let lines = lines.map(|l| l.unwrap()).collect::<Vec<_>>();

    let nodes = lines.iter().map(|l| parse(l)).collect::<HashMap<_, _>>();

    let s0 = State {
        instruction_idx: 0,
        node: "AAA".to_string(),
        steps: 0,
    };
    let mut s = s0;
    while s.node != "ZZZ" {
        s = process(&s, &instructions, &nodes);
    }
    let r1 = s.steps;
    println!("{}", r1);
    let l = instructions.len();

    let starting_nodes = nodes
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let r2 = starting_nodes
        .iter()
        .map(|n| {
            let mut s = State {
                instruction_idx: 0,
                node: n.to_string(),
                steps: 0,
            };
            while !s.node.ends_with('Z') {
                s = process(&s, &instructions, &nodes);
            }
            s.steps / l
        })
        .product::<usize>()
        * l;
    println!("{}", r2);
}
