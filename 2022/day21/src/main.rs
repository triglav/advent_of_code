use std::{collections::HashMap, io};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation<'a> {
    Plus(&'a str, &'a str),
    Minus(&'a str, &'a str),
    Multiply(&'a str, &'a str),
    Divide(&'a str, &'a str),
    Value(i64),
}

fn parse_monkey<'a>(s: &'a String) -> (&'a str, Operation<'a>) {
    let t0: Vec<_> = s.split(':').collect();
    let name = t0[0];
    let t: Vec<_> = t0[1].trim().split(' ').collect();
    if t.len() == 1 {
        return (name, Operation::Value(t[0].parse::<i64>().unwrap()));
    }
    let op = match t[1] {
        "+" => Operation::Plus(t[0], t[2]),
        "-" => Operation::Minus(t[0], t[2]),
        "*" => Operation::Multiply(t[0], t[2]),
        "/" => Operation::Divide(t[0], t[2]),
        _ => panic!("Invalid monkey job {}", t[1]),
    };
    (name, op)
}

fn evaluate(name: &str, operations: &HashMap<&str, Operation>) -> i64 {
    let op = operations.get(name).unwrap();
    match *op {
        Operation::Plus(a, b) => evaluate(a, operations) + evaluate(b, operations),
        Operation::Minus(a, b) => evaluate(a, operations) - evaluate(b, operations),
        Operation::Multiply(a, b) => evaluate(a, operations) * evaluate(b, operations),
        Operation::Divide(a, b) => evaluate(a, operations) / evaluate(b, operations),
        Operation::Value(a) => a,
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let operations: HashMap<&str, Operation> = lines.iter().map(|l| parse_monkey(&l)).collect();

    let r1 = evaluate("root", &operations);
    println!("{:?}", r1);
}
