use std::io;

type Section = (u8, u8);

fn parse_sections(s: &str) -> Section {
    let t = s
        .split('-')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    assert_eq!(t.len(), 2);
    (t[0], t[1])
}

fn sections_overlap(s1: Section, s2: Section) -> bool {
    let (a, b) = if s1.0 < s2.0 {
        (s1, s2)
    } else if s1.0 > s2.0 {
        (s2, s1)
    } else if s1.1 >= s2.1 {
        (s1, s2)
    } else {
        (s2, s1)
    };
    a.0 <= b.0 && a.1 >= b.1
}

fn main() {
    let r1 = io::stdin()
        .lines()
        .map(|l| {
            let s = l
                .unwrap()
                .split(',')
                .map(parse_sections)
                .collect::<Vec<Section>>();
            sections_overlap(s[0], s[1])
        })
        .filter(|r| *r == true)
        .count();
    println!("{}", r1);
}
