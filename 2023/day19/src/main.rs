use std::{
    collections::{HashMap, VecDeque},
    io,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
struct PartCombinations {
    parts: HashMap<PartType, (u32, u32)>,
}

impl PartCombinations {
    pub fn new() -> Self {
        Self {
            parts: HashMap::from([
                (PartType::X, (1, 4000)),
                (PartType::M, (1, 4000)),
                (PartType::A, (1, 4000)),
                (PartType::S, (1, 4000)),
            ]),
        }
    }

    pub fn get(&self, part_type: PartType) -> (u32, u32) {
        *self.parts.get(&part_type).expect("Unknown part type")
    }

    fn clone_with(&self, part_type: PartType, value: (u32, u32)) -> Self {
        let mut parts = self.parts.clone();
        parts.entry(part_type).and_modify(|e| *e = value);
        Self { parts }
    }

    pub fn split_by_less_than(
        &self,
        part_type: PartType,
        value: u32,
    ) -> (Option<PartCombinations>, Option<PartCombinations>) {
        let (l, r) = self.get(part_type);
        let left = (l, r.min(value - 1));
        let right = (l.max(value), r);

        let accepted = if left.0 <= left.1 {
            Some(self.clone_with(part_type, left))
        } else {
            None
        };
        let rejected = if right.0 <= right.1 {
            Some(self.clone_with(part_type, right))
        } else {
            None
        };
        (accepted, rejected)
    }

    pub fn split_by_greater_than(
        &self,
        part_type: PartType,
        value: u32,
    ) -> (Option<PartCombinations>, Option<PartCombinations>) {
        let (rejected, accepted) = self.split_by_less_than(part_type, value + 1);
        (accepted, rejected)
    }

    pub fn split_by_condition(
        &self,
        condition: &Condition,
    ) -> (Option<PartCombinations>, Option<PartCombinations>) {
        match condition {
            Condition::LessThan(t, v) => self.split_by_less_than(*t, *v),
            Condition::GreaterThan(t, v) => self.split_by_greater_than(*t, *v),
            Condition::Nope => (Some(self.clone()), None),
        }
    }

    pub fn count_combinations(&self) -> u64 {
        self.parts
            .values()
            .map(|(l, r)| (r - l + 1) as u64)
            .product()
    }
}

fn split_part_combinations(workflows: &HashMap<String, Vec<Rule>>) -> Vec<PartCombinations> {
    let mut accepted = Vec::new();

    let mut todo = VecDeque::from([("in", PartCombinations::new())]);
    while let Some((location, combinations)) = todo.pop_front() {
        if location == "A" {
            accepted.push(combinations);
            continue;
        }
        if location == "R" {
            continue;
        }

        let workflow = workflows.get(location).unwrap();
        let mut combinations = combinations;
        for rule in workflow {
            let (accepted, rejected) = combinations.split_by_condition(&rule.condition);
            if let Some(accepted) = accepted {
                todo.push_back((&rule.destination, accepted));
            }
            if let Some(rejected) = rejected {
                combinations = rejected;
            }
        }
    }
    accepted
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

    let r2 = split_part_combinations(&workflows)
        .into_iter()
        .map(|c| c.count_combinations())
        .sum::<u64>();
    println!("{}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_by_less_than() {
        let c = PartCombinations::new();

        let (a, r) = c.split_by_less_than(PartType::X, 500);
        let a = a.unwrap();
        let r = r.unwrap();

        assert_eq!(a.get(PartType::X), (1, 499));
        assert_eq!(a.get(PartType::M), (1, 4000));
        assert_eq!(a.get(PartType::A), (1, 4000));
        assert_eq!(a.get(PartType::S), (1, 4000));

        assert_eq!(r.get(PartType::X), (500, 4000));
        assert_eq!(r.get(PartType::M), (1, 4000));
        assert_eq!(r.get(PartType::A), (1, 4000));
        assert_eq!(r.get(PartType::S), (1, 4000));

        let (a, r) = a.split_by_less_than(PartType::M, 1234);
        let a = a.unwrap();
        let r = r.unwrap();

        assert_eq!(a.get(PartType::X), (1, 499));
        assert_eq!(a.get(PartType::M), (1, 1233));
        assert_eq!(a.get(PartType::A), (1, 4000));
        assert_eq!(a.get(PartType::S), (1, 4000));

        assert_eq!(r.get(PartType::X), (1, 499));
        assert_eq!(r.get(PartType::M), (1234, 4000));
        assert_eq!(r.get(PartType::A), (1, 4000));
        assert_eq!(r.get(PartType::S), (1, 4000));

        let (a2, r2) = a.split_by_less_than(PartType::X, 2000);
        let a2 = a2.unwrap();

        assert_eq!(a2.get(PartType::X), (1, 499));
        assert_eq!(a2.get(PartType::M), (1, 1233));
        assert_eq!(a2.get(PartType::A), (1, 4000));
        assert_eq!(a2.get(PartType::S), (1, 4000));

        assert!(r2.is_none());

        let (a2, r2) = r.split_by_less_than(PartType::M, 1000);
        let r2 = r2.unwrap();

        assert!(a2.is_none());

        assert_eq!(r2.get(PartType::X), (1, 499));
        assert_eq!(r2.get(PartType::M), (1234, 4000));
        assert_eq!(r2.get(PartType::A), (1, 4000));
        assert_eq!(r2.get(PartType::S), (1, 4000));
    }

    #[test]
    fn test_split_by_greater_than() {
        let c = PartCombinations::new();

        let (a, r) = c.split_by_greater_than(PartType::X, 500);
        let a = a.unwrap();
        let r = r.unwrap();

        assert_eq!(a.get(PartType::X), (501, 4000));
        assert_eq!(a.get(PartType::M), (1, 4000));
        assert_eq!(a.get(PartType::A), (1, 4000));
        assert_eq!(a.get(PartType::S), (1, 4000));

        assert_eq!(r.get(PartType::X), (1, 500));
        assert_eq!(r.get(PartType::M), (1, 4000));
        assert_eq!(r.get(PartType::A), (1, 4000));
        assert_eq!(r.get(PartType::S), (1, 4000));

        let (a, r) = a.split_by_greater_than(PartType::M, 1234);
        let a = a.unwrap();
        let r = r.unwrap();

        assert_eq!(a.get(PartType::X), (501, 4000));
        assert_eq!(a.get(PartType::M), (1235, 4000));
        assert_eq!(a.get(PartType::A), (1, 4000));
        assert_eq!(a.get(PartType::S), (1, 4000));

        assert_eq!(r.get(PartType::X), (501, 4000));
        assert_eq!(r.get(PartType::M), (1, 1234));
        assert_eq!(r.get(PartType::A), (1, 4000));
        assert_eq!(r.get(PartType::S), (1, 4000));

        let (a2, r2) = a.split_by_greater_than(PartType::X, 200);
        let a2 = a2.unwrap();

        assert_eq!(a2.get(PartType::X), (501, 4000));
        assert_eq!(a2.get(PartType::M), (1235, 4000));
        assert_eq!(a2.get(PartType::A), (1, 4000));
        assert_eq!(a2.get(PartType::S), (1, 4000));

        assert!(r2.is_none());

        let (a2, r2) = r.split_by_greater_than(PartType::M, 2000);
        let r2 = r2.unwrap();

        assert!(a2.is_none());

        assert_eq!(r2.get(PartType::X), (501, 4000));
        assert_eq!(r2.get(PartType::M), (1, 1234));
        assert_eq!(r2.get(PartType::A), (1, 4000));
        assert_eq!(r2.get(PartType::S), (1, 4000));
    }

    #[test]
    fn test_count_combinations() {
        let c = PartCombinations {
            parts: HashMap::from([
                (PartType::X, (1, 1)),
                (PartType::M, (1, 1)),
                (PartType::A, (1, 1)),
                (PartType::S, (1, 1)),
            ]),
        };
        assert_eq!(c.count_combinations(), 1);

        let c = PartCombinations {
            parts: HashMap::from([
                (PartType::X, (1, 2)),
                (PartType::M, (1, 1)),
                (PartType::A, (1, 1)),
                (PartType::S, (1, 1)),
            ]),
        };
        assert_eq!(c.count_combinations(), 2);

        let c = PartCombinations {
            parts: HashMap::from([
                (PartType::X, (2, 2)),
                (PartType::M, (1, 1)),
                (PartType::A, (1, 1)),
                (PartType::S, (1, 1)),
            ]),
        };
        assert_eq!(c.count_combinations(), 1);

        let c = PartCombinations {
            parts: HashMap::from([
                (PartType::X, (1, 100)),
                (PartType::M, (1, 1)),
                (PartType::A, (1, 1)),
                (PartType::S, (1, 1)),
            ]),
        };
        assert_eq!(c.count_combinations(), 100);

        let c = PartCombinations {
            parts: HashMap::from([
                (PartType::X, (1, 2)),
                (PartType::M, (1, 2)),
                (PartType::A, (1, 2)),
                (PartType::S, (1, 2)),
            ]),
        };
        assert_eq!(c.count_combinations(), 16);
    }
}
