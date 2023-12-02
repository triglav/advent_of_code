use std::io;

#[derive(Default)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

fn parse(line: &str) -> (u32, Vec<Set>) {
    let t0 = line.split(':').collect::<Vec<_>>();
    let id = t0[0][5..].parse::<u32>().unwrap();
    let sets = t0[1]
        .split(';')
        .map(|cubes| {
            let set = cubes.split(',').fold(Set::default(), |mut set, cube| {
                let t1 = cube.trim().split(' ').collect::<Vec<_>>();
                let n = t1[0].parse::<u32>().unwrap();
                let color = t1[1];
                match color {
                    "red" => set.red = n,
                    "green" => set.green = n,
                    "blue" => set.blue = n,
                    _ => panic!(),
                };
                set
            });
            set
        })
        .collect::<Vec<_>>();
    (id, sets)
}

fn main() {
    let limit = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let r1 = io::stdin()
        .lines()
        .map(|l| parse(l.unwrap().as_str()))
        .filter_map(|(id, sets)| {
            let m = sets.iter().fold(Set::default(), |mut a, set| {
                a.red = a.red.max(set.red);
                a.green = a.green.max(set.green);
                a.blue = a.blue.max(set.blue);
                a
            });
            if m.red <= limit.red && m.green <= limit.green && m.blue <= limit.blue {
                Some(id)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("{}", r1);
}
