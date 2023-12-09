use std::io;

fn parse(s: &str) -> Vec<u64> {
    s.split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
}

fn count_ways_to_win(time: u64, distance: u64) -> u64 {
    let r = (0..=time)
        .map(|charging_time| {
            let speed = charging_time;
            let remaining_time = time - charging_time;
            speed * remaining_time
        })
        .filter(|&x| x > distance);
    r.count() as u64
}

fn fix_number(v: &[u64]) -> u64 {
    v.iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let mut lines = io::stdin().lines();
    let time = parse(lines.next().unwrap().unwrap().as_str());
    let distance = parse(lines.next().unwrap().unwrap().as_str());
    assert_eq!(time.len(), distance.len());

    let r1 = time
        .iter()
        .zip(distance.iter())
        .map(|(&time, &distance)| count_ways_to_win(time, distance))
        .product::<u64>();
    println!("{}", r1);

    let time = fix_number(&time);
    let distance = fix_number(&distance);
    let r2 = count_ways_to_win(time, distance);
    println!("{}", r2);
}
