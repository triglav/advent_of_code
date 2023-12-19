use std::{collections::HashMap, io};

#[derive(Debug, Clone, Copy)]
enum PartType {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Condition {
    LessThan(PartType, u32),
    GreaterThan(PartType, u32),
    Nope,
}

#[derive(Debug)]
struct Rule {
    condition: Condition,
    destination: String,
}

fn parse_workflows(lines: &mut impl Iterator<Item = String>) -> HashMap<String, Vec<Rule>> {
    let mut r = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let t = line.split('{').collect::<Vec<_>>();
        assert_eq!(t.len(), 2);
        let name = t[0].to_string();
        let t = t[1].split('}').next().unwrap();
        let rules = t.split(',').map(|s| {
            let t = s.split(':').collect::<Vec<_>>();
            assert!(t.len() == 2 || t.len() == 1);

            let (condition, destination) = if t.len() == 2 {
                let mut c = t[0].chars();
                let part = c.next().unwrap();
                let part = match part {
                    'x' => PartType::X,
                    'm' => PartType::M,
                    'a' => PartType::A,
                    's' => PartType::S,
                    _ => panic!("Unknown part {}", part),
                };
                let op = c.next().unwrap();
                let value = t[0][2..].parse::<u32>().unwrap();
                let condition = match op {
                    '<' => Condition::LessThan(part, value),
                    '>' => Condition::GreaterThan(part, value),
                    _ => panic!("Unknown condition {}", op),
                };
                (condition, t[1].to_string())
            } else {
                (Condition::Nope, t[0].to_string())
            };
            Rule {
                condition,
                destination,
            }
        });
        r.insert(name, rules.collect());
    }
    r
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn parse_parts(lines: &mut impl Iterator<Item = String>) -> Vec<Part> {
    let mut r = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }

        let p = line[1..line.len() - 1].split(',').map(|s| s.trim()).fold(
            Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            },
            |mut p, s| {
                let mut t = s.split('=');
                let key = t.next().unwrap();
                let value = t.next().unwrap().parse::<u32>().unwrap();
                match key {
                    "x" => p.x = value,
                    "m" => p.m = value,
                    "a" => p.a = value,
                    "s" => p.s = value,
                    _ => panic!("Unknown key {}", key),
                };
                p
            },
        );
        r.push(p);
    }
    r
}

fn get_part_value(part: &Part, part_type: PartType) -> u32 {
    match part_type {
        PartType::X => part.x,
        PartType::M => part.m,
        PartType::A => part.a,
        PartType::S => part.s,
    }
}

fn test_condition(condition: &Condition, part: &Part) -> bool {
    match condition {
        Condition::LessThan(t, v) => get_part_value(part, *t) < *v,
        Condition::GreaterThan(t, v) => get_part_value(part, *t) > *v,
        Condition::Nope => true,
    }
}

fn evaluate_a_workflow<'a>(workflow: &'a [Rule], part: &Part) -> &'a str {
    workflow
        .iter()
        .find(|r| test_condition(&r.condition, part))
        .expect("No rule matches")
        .destination
        .as_str()
}

fn evaluate_workflows(workflows: &HashMap<String, Vec<Rule>>, part: &Part) -> bool {
    // print!("{:?}: ", part);
    let mut location = "in";
    while location != "A" && location != "R" {
        // print!("{} -> ", location);
        let workflow = workflows.get(location).unwrap();
        location = evaluate_a_workflow(workflow, part);
    }
    // println!("{}", location);
    location == "A"
}

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap());

    let workflows = parse_workflows(&mut lines);
    let parts = parse_parts(&mut lines);

    let r1 = parts
        .into_iter()
        .filter_map(|p| evaluate_workflows(&workflows, &p).then_some(p.x + p.m + p.a + p.s))
        .sum::<u32>();
    println!("{}", r1);
}
