use std::io;

fn parse(s: &str) -> Vec<u32> {
    s.split(' ')
        .filter_map(|x| x.parse::<u32>().ok())
        .collect::<Vec<_>>()
}

fn count_ways_to_win(time: u32, distance: u32) -> u32 {
    let r = (0..=time)
        .map(|charging_time| {
            let speed = charging_time;
            let remaining_time = time - charging_time;
            speed * remaining_time
        })
        .filter(|&x| x > distance);
    r.count() as u32
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
        .product::<u32>();
    println!("{}", r1);
}
