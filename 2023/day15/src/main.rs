use std::{collections::HashMap, fmt, io};

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as u8)
        .fold(0u32, |a, c| ((a + c as u32) * 17) % 256) as u8
}

enum Step<'a> {
    Remove(&'a str),
    Add(&'a str, u32),
}

fn parse_step(s: &str) -> Step {
    let is_remove = s.contains('-');
    let is_add = s.contains('=');
    assert_ne!(is_remove, is_add);

    if is_remove {
        let t = s.split('-').collect::<Vec<_>>();
        let label = t[0];
        return Step::Remove(label);
    }
    if is_add {
        let t = s.split('=').collect::<Vec<_>>();
        let label = t[0];
        let focal_length = t[1].parse().unwrap();
        return Step::Add(label, focal_length);
    }
    panic!("Invalid step: {}", s);
}

#[derive(Clone, Copy)]
struct Lense<'a> {
    pub label: &'a str,
    pub focal_length: u32,
}

#[derive(Default, Clone)]
struct LenseBox<'a> {
    pub lenses: Vec<Lense<'a>>,
}

impl fmt::Display for LenseBox<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in self.lenses.iter() {
            write!(f, "[{} {}] ", l.label, l.focal_length)?;
        }
        Ok(())
    }
}

struct Boxes<'a> {
    boxes: HashMap<u8, LenseBox<'a>>,
}

impl fmt::Display for Boxes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (box_num, lense_box) in self.boxes.iter() {
            writeln!(f, "Box {}: {} ", box_num, lense_box)?;
        }
        Ok(())
    }
}

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self {
            boxes: HashMap::new(),
        }
    }

    fn get_box(&mut self, label: &str) -> &mut LenseBox<'a> {
        let box_num = hash(label);
        self.boxes.entry(box_num).or_default()
    }

    fn execute_step(&mut self, step: Step<'a>) {
        match step {
            Step::Remove(label) => {
                let b = self.get_box(label);
                b.lenses.retain(|l| l.label != label);
            }
            Step::Add(label, focal_length) => {
                let b = self.get_box(label);
                match b.lenses.iter().position(|l| l.label == label) {
                    Some(i) => b.lenses[i].focal_length = focal_length,
                    None => b.lenses.push(Lense {
                        label,
                        focal_length,
                    }),
                }
            }
        }
    }

    fn get_focusing_power(&self) -> u32 {
        self.boxes
            .iter()
            .map(|(&box_num, b)| {
                b.lenses
                    .iter()
                    .enumerate()
                    .map(|(slot_number, lense)| {
                        (box_num as u32 + 1) * (slot_number as u32 + 1) * lense.focal_length
                    })
                    .sum::<u32>()
            })
            .sum()
    }
}

fn main() {
    let line = io::stdin().lines().map(|l| l.unwrap()).next().unwrap();
    let steps = line.split(',').collect::<Vec<_>>();

    let r1 = steps.iter().map(|s| hash(s) as u32).sum::<u32>();
    println!("{}", r1);

    let mut boxes = Boxes::new();
    steps
        .into_iter()
        .map(parse_step)
        .for_each(|s| boxes.execute_step(s));

    let r2 = boxes.get_focusing_power();
    println!("{}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("HASH", 52)]
    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("qp=3", 97)]
    #[test_case("cm=2", 47)]
    #[test_case("qp-", 14)]
    #[test_case("pc=4", 180)]
    #[test_case("ot=9", 9)]
    #[test_case("ab=5", 197)]
    #[test_case("pc-", 48)]
    #[test_case("pc=6", 214)]
    #[test_case("ot=7", 231)]
    fn test_hash(s: &str, expected: u8) {
        let result = hash(s);
        assert_eq!(expected, result);
    }
}
