use std::io;

fn wrap(v: i32, count: i32) -> i32 {
    let v0 = v % count;
    let v2 = if v0 < 0 { v0 + count } else { v0 };
    assert_eq!(v2, v.rem_euclid(count));
    v2
}

#[derive(Clone, Copy, Debug)]
struct Item {
    priority: usize,
    value: i32,
}

fn rotate(priority: usize, numbers: &mut Vec<Item>) {
    // priority, idx, value
    let pos = numbers
        .iter()
        .position(|item| priority == item.priority)
        .unwrap();
    let item = numbers.remove(pos);
    let count = numbers.len() as i32;
    let new_pos = wrap(pos as i32 + item.value, count);
    numbers.insert(new_pos as usize, item);
}

fn main() {
    let mut numbers: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .enumerate()
        .map(|(index, value)| Item {
            priority: index,
            value,
        })
        .collect();

    let count = numbers.len();
    for i in 0..count {
        rotate(i, &mut numbers);
    }
    let zero_pos = numbers.iter().position(|i| i.value == 0).unwrap();

    let r1 = [1000, 2000, 3000]
        .map(|idx| (zero_pos + idx) % count)
        .map(|p| numbers.get(p).unwrap().value)
        .iter()
        .sum::<i32>();
    println!("{:?}", r1);
}
