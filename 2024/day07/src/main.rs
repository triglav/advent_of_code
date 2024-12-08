use std::{collections::VecDeque, io};

enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Equation<T = u64> {
    left: T,
    right: VecDeque<T>,
}

impl Equation {
    pub fn execute(&self, op: Operator) -> Equation {
        let mut e = self.clone();
        let a = e.right.pop_front().unwrap();
        let b = e.right.pop_front().unwrap();
        let r = match op {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
        };
        e.right.push_front(r);
        e
    }
}

fn parse(s: &str) -> Equation {
    let t = s.split(": ").collect::<Vec<_>>();
    let left = t[0].parse::<u64>().unwrap();
    let right = t[1]
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect();
    Equation { left, right }
}

fn solve(e: &Equation) -> bool {
    let mut todo = vec![e.clone()];
    while let Some(e) = todo.pop() {
        if e.right.len() == 1 {
            if e.left == e.right[0] {
                return true;
            }
            continue;
        }
        if e.right.is_empty() {
            panic!("Invalid equation: {:?}", e);
        }
        todo.push(e.execute(Operator::Add));
        todo.push(e.execute(Operator::Multiply));
    }
    false
}

fn main() {
    let equations = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();

    let r1 = equations
        .iter()
        .filter(|e| solve(e))
        .map(|e| e.left)
        .sum::<u64>();
    println!("{}", r1);
}
