use std::io;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

#[derive(Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    op: Operation,
    test: u64,
    throw_true: usize,
    throw_false: usize,
}

impl Monkey {
    fn inspect(&self, i: usize, relief: Option<u64>) -> (usize, u64) {
        let old = self.items[i];
        // inspect
        let new0 = match self.op {
            Operation::Add(v) => old + v,
            Operation::Mul(v) => old * v,
            Operation::Square => old * old,
        };
        let new = match relief {
            None => new0 / 3,
            Some(v) => new0 % v,
        };
        // worry level
        let throw_to = match new % self.test {
            0 => self.throw_true,
            _ => self.throw_false,
        };
        (throw_to, new)
    }
}

fn parse_operation(s: &String) -> Operation {
    let t = s.trim().split(' ').collect::<Vec<&str>>();
    match t[4] {
        "+" => {
            let v = t[5].parse::<u64>().unwrap();
            Operation::Add(v)
        }
        "*" => match t[5] {
            "old" => Operation::Square,
            _ => {
                let v = t[5].parse::<u64>().unwrap();
                Operation::Mul(v)
            }
        },
        _ => panic!("Invalid operation {}", t[4]),
    }
}

fn parse_only_number(s: &str) -> u64 {
    s.trim_matches(|c: char| !c.is_digit(10))
        .parse::<u64>()
        .unwrap()
}

fn parse_monkey(lines: &[String]) -> Monkey {
    let id = parse_only_number(lines[0].as_str()) as usize;
    let items = lines[1]
        .as_str()
        .trim_matches(|c: char| !c.is_digit(10))
        .split(',')
        .map(|t| t.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let op = parse_operation(&lines[2]);
    let test = parse_only_number(lines[3].as_str());
    let throw_true = parse_only_number(lines[4].as_str()) as usize;
    let throw_false = parse_only_number(lines[5].as_str()) as usize;
    Monkey {
        id,
        items,
        op,
        test,
        throw_true,
        throw_false,
    }
}

fn do_round(monkeys0: &Vec<Monkey>, activity: &mut Vec<usize>, relief: Option<u64>) -> Vec<Monkey> {
    let mut r = (*monkeys0).clone();
    for i in 0..r.len() {
        for item_idx in 0..r[i].items.len() {
            activity[i] += 1;
            let (throw_to, item) = r[i].inspect(item_idx, relief);
            assert_eq!(r[throw_to].id, throw_to);
            r[throw_to].items.push(item);
        }
        r[i].items.clear();
    }
    r
}

fn get_monkey_business(activity: &Vec<usize>) -> usize {
    let mut a = activity.clone();
    a.sort();
    a.iter().rev().take(2).product::<usize>()
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let monkeys = lines
        .split(|l| l.is_empty())
        .map(parse_monkey)
        .collect::<Vec<Monkey>>();

    let mut activity1 = vec![0; monkeys.len()];
    let mut monkeys1 = monkeys.clone();
    for _ in 0..20 {
        monkeys1 = do_round(&monkeys1, &mut activity1, None);
    }
    let r1 = get_monkey_business(&activity1);
    println!("{}", r1);

    let relief = monkeys.iter().map(|m| m.test).product::<u64>();

    let mut activity2 = vec![0; monkeys.len()];
    let mut monkeys2 = monkeys.clone();
    for _ in 0..10000 {
        monkeys2 = do_round(&monkeys2, &mut activity2, Some(relief));
    }
    let r2 = get_monkey_business(&activity2);
    println!("{}", r2);
}
