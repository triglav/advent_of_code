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

fn order_sections(s1: Section, s2: Section) -> (Section, Section) {
    if s1.0 < s2.0 {
        (s1, s2)
    } else if s1.0 > s2.0 {
        (s2, s1)
    } else if s1.1 >= s2.1 {
        (s1, s2)
    } else {
        (s2, s1)
    }
}

fn sections_overlap_fully(s1: Section, s2: Section) -> bool {
    s1.0 <= s2.0 && s1.1 >= s2.1
}

fn sections_overlap_at_all(s1: Section, s2: Section) -> bool {
    s2.0 <= s1.1
}

fn main() {
    let (r1, r2) = io::stdin()
        .lines()
        .map(|l| {
            let s = l
                .unwrap()
                .split(',')
                .map(parse_sections)
                .collect::<Vec<Section>>();
            let (s1, s2) = order_sections(s[0], s[1]);
            (
                sections_overlap_fully(s1, s2),
                sections_overlap_at_all(s1, s2),
            )
        })
        .fold((0, 0), |(mut r1, mut r2), (c1, c2)| {
            if c1 {
                r1 += 1;
            }
            if c2 {
                r2 += 1;
            }
            (r1, r2)
        });
    println!("{}", r1);
    println!("{}", r2);
}
