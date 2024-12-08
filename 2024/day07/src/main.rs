use std::{collections::VecDeque, io};

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
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
            Operator::Concat => format!("{}{}", a, b).parse::<u64>().unwrap(),
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

fn solve(e: &Equation, operators: &[Operator]) -> bool {
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
        operators.iter().for_each(|op| todo.push(e.execute(*op)))
    }
    false
}

fn main() {
    let equations = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .collect::<Vec<_>>();

    let (e_1, e_2): (Vec<_>, Vec<_>) = equations
        .into_iter()
        .partition(|e| solve(e, [Operator::Add, Operator::Multiply].as_ref()));

    let r1 = e_1.iter().map(|e| e.left).sum::<u64>();
    println!("{}", r1);

    let r2b = e_2
        .iter()
        .filter(|e| {
            solve(
                e,
                [Operator::Add, Operator::Multiply, Operator::Concat].as_ref(),
            )
        })
        .map(|e| e.left)
        .sum::<u64>();
    let r2 = r1 + r2b;
    println!("{}", r2);
}
