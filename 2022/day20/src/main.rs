use std::io;

fn wrap(v: i64, count: i64) -> i64 {
    let v0 = v % count;
    let v2 = if v0 < 0 { v0 + count } else { v0 };
    assert_eq!(v2, v.rem_euclid(count));
    v2
}

#[derive(Clone, Copy, Debug)]
struct Item {
    priority: usize,
    value: i64,
}

fn rotate(priority: usize, numbers: &mut Vec<Item>) {
    // priority, idx, value
    let pos = numbers
        .iter()
        .position(|item| priority == item.priority)
        .unwrap();
    let item = numbers.remove(pos);
    let count = numbers.len() as i64;
    let new_pos = wrap(pos as i64 + item.value, count);
    numbers.insert(new_pos as usize, item);
}

fn main() {
    let mut numbers: Vec<_> = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .enumerate()
        .map(|(index, value)| Item {
            priority: index,
            value,
        })
        .collect();
    let decription_key = 811589153;
    let mut numbers2 = numbers
        .clone()
        .into_iter()
        .map(|i| Item {
            priority: i.priority,
            value: i.value * decription_key,
        })
        .collect();

    let count = numbers.len();
    for i in 0..count {
        rotate(i, &mut numbers);
    }
    let zero_pos1 = numbers.iter().position(|i| i.value == 0).unwrap();
    let r1 = [1000, 2000, 3000]
        .map(|idx| (zero_pos1 + idx) % count)
        .map(|p| numbers.get(p).unwrap().value)
        .iter()
        .sum::<i64>();
    println!("{:?}", r1);

    for _ in 0..10 {
        for i in 0..count {
            rotate(i, &mut numbers2);
        }
    }
    let zero_pos2 = numbers2.iter().position(|i| i.value == 0).unwrap();
    let r2 = [1000, 2000, 3000]
        .map(|idx| (zero_pos2 + idx) % count)
        .map(|p| numbers2.get(p).unwrap().value)
        .iter()
        .sum::<i64>();
    println!("{:?}", r2);
}
