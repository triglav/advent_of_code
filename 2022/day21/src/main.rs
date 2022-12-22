use std::{
    collections::{HashMap, HashSet},
    io,
};

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

fn evaluate<'a>(
    name: &'a str,
    ops: &'a HashMap<&str, Operation>,
    res: &mut HashMap<&'a str, i64>,
) -> i64 {
    let op = ops.get(name).unwrap();
    let r = match *op {
        Operation::Plus(a, b) => evaluate(a, ops, res) + evaluate(b, ops, res),
        Operation::Minus(a, b) => evaluate(a, ops, res) - evaluate(b, ops, res),
        Operation::Multiply(a, b) => evaluate(a, ops, res) * evaluate(b, ops, res),
        Operation::Divide(a, b) => evaluate(a, ops, res) / evaluate(b, ops, res),
        Operation::Value(a) => a,
    };
    res.insert(name, r);
    r
}

fn get_op_params<'a>(op: &'a Operation) -> Option<(&'a str, &'a str)> {
    match op {
        Operation::Plus(a, b) => Some((a, b)),
        Operation::Minus(a, b) => Some((a, b)),
        Operation::Multiply(a, b) => Some((a, b)),
        Operation::Divide(a, b) => Some((a, b)),
        Operation::Value(_) => None,
    }
}

fn map_humn_branch<'a>(name: &'a str, ops: &'a HashMap<&str, Operation>) -> Vec<&'a str> {
    fn map_humn_branch_rec<'b>(
        name: &'b str,
        ops: &'b HashMap<&str, Operation>,
        humn: &mut Vec<&'b str>,
    ) -> bool {
        if name == "humn" {
            humn.push(name);
            return true;
        }
        let op = ops.get(name).unwrap();
        match get_op_params(op) {
            Some((a, b)) => {
                if map_humn_branch_rec(a, ops, humn) || map_humn_branch_rec(b, ops, humn) {
                    humn.push(name);
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
    let mut humn_branch = Vec::<&str>::new();
    map_humn_branch_rec(name, ops, &mut humn_branch);
    humn_branch
}

fn identify_unknown<'a>((a, b): (&'a str, &'a str), humn: &'a HashSet<&str>) -> (&'a str, &'a str) {
    if !humn.contains(a) {
        assert!(humn.contains(b));
        (a, b)
    } else {
        assert!(!humn.contains(b));
        (b, a)
    }
}

fn find_humn_value(
    expected: i64,
    to_solve: &str,
    ops: &HashMap<&str, Operation>,
    res: &HashMap<&str, i64>,
    humn: &HashSet<&str>,
) -> i64 {
    if to_solve == "humn" {
        return expected;
    }
    match ops.get(to_solve).unwrap() {
        Operation::Plus(a, b) => {
            let (known, unknown) = identify_unknown((*a, *b), humn);
            let known_value = res.get(known).unwrap();
            let unknown_value = expected - known_value;
            find_humn_value(unknown_value, unknown, ops, res, humn)
        }
        Operation::Minus(a, b) => {
            // r = a - b
            if humn.contains(a) {
                // a = r + b
                assert!(!humn.contains(b));
                let b_value = res.get(b).unwrap();
                let a_value = expected + b_value;
                find_humn_value(a_value, a, ops, res, humn)
            } else {
                // b = a - r
                assert!(humn.contains(b));
                let a_value = res.get(a).unwrap();
                let b_value = a_value - expected;
                find_humn_value(b_value, b, ops, res, humn)
            }
        }
        Operation::Multiply(a, b) => {
            let (known, unknown) = identify_unknown((*a, *b), humn);
            let known_value = res.get(known).unwrap();
            let unknown_value = expected / known_value;
            find_humn_value(unknown_value, unknown, ops, res, humn)
        }
        Operation::Divide(a, b) => {
            // r = a / b
            if humn.contains(a) {
                // a = r * b
                assert!(!humn.contains(b));
                let b_value = res.get(b).unwrap();
                let a_value = expected * b_value;
                find_humn_value(a_value, a, ops, res, humn)
            } else {
                // b = a / r
                assert!(humn.contains(b));
                let a_value = res.get(a).unwrap();
                let b_value = a_value / expected;
                find_humn_value(b_value, b, ops, res, humn)
            }
        }
        Operation::Value(_) => panic!("Unexpected opration!"),
    }
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|l| l.unwrap()).collect();
    let operations: HashMap<&str, Operation> = lines.iter().map(|l| parse_monkey(&l)).collect();

    let mut results = HashMap::<&str, i64>::new();

    let r1 = evaluate("root", &operations, &mut results);
    println!("{}", r1);

    let root = operations.get("root").unwrap();
    let (root_a, root_b) = get_op_params(root).unwrap();

    let humn_branch = map_humn_branch(root_a, &operations)
        .into_iter()
        .collect::<HashSet<&str>>();

    let (known, unknown) = identify_unknown((root_a, root_b), &&humn_branch);
    let expected = *results.get(known).unwrap();
    let r2 = find_humn_value(expected, unknown, &operations, &results, &humn_branch);
    println!("{}", r2);
}
