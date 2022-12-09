use std::io;

type Stack = Vec<char>;

fn parse_config_line(s: &String, stack_count: usize) -> Vec<char> {
    (0..stack_count)
        .map(|n| 4 * n + 1)
        .map(|idx| s.chars().nth(idx).unwrap())
        .collect()
}

fn parse_procedure_line(s: &String) -> (usize, usize, usize) {
    let v: Vec<_> = s.split(' ').collect();
    (
        v[1].parse::<usize>().unwrap(),
        v[3].parse::<usize>().unwrap() - 1,
        v[5].parse::<usize>().unwrap() - 1,
    )
}

fn rearrange(stacks: &mut Vec<Stack>, count: usize, from_idx: usize, to_idx: usize) {
    (0..count).for_each(|_| {
        let c = stacks[from_idx].pop().unwrap();
        stacks[to_idx].push(c);
    });
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let idx = lines.iter().position(|l| l == "").unwrap();
    let stack_count = (lines[idx - 1].len() + 1) / 4;

    let mut stacks = (0..=(idx - 2))
        .rev()
        .map(|i| &lines[i])
        .map(|l| parse_config_line(l, stack_count))
        .fold(vec![Stack::new(); stack_count], |mut a, s| {
            s.iter().enumerate().for_each(|(idx, c)| {
                if *c != ' ' {
                    a[idx].push(*c);
                }
            });
            a
        });
    ((idx + 1)..lines.len())
        .map(|idx| parse_procedure_line(&lines[idx]))
        .for_each(|(count, from_idx, to_idx)| {
            rearrange(&mut stacks, count, from_idx, to_idx);
        });
    let r1 = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
    println!("{}", r1);
}
