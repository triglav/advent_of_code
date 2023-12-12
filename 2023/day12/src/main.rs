use std::{collections::VecDeque, io};

fn verify_arrangement(s: &str, contigous_list: &[u32]) -> bool {
    let t = s
        .split('.')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.len() as u32)
            }
        })
        .collect::<Vec<u32>>();
    t.len() == contigous_list.len() && t.into_iter().zip(contigous_list).all(|(a, &b)| a == b)
}

fn verify_arrangement_partial(s: &str, contigous_list: &[u32]) -> bool {
    let first_unknown = s.find('?');
    if first_unknown.is_none() {
        return verify_arrangement(s, contigous_list);
    }

    let first_unknown = first_unknown.unwrap();
    let skip_last = s.chars().nth(first_unknown - 1) == Some('#');

    let t = s[0..first_unknown]
        .split('.')
        .filter_map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.len() as u32)
            }
        })
        .collect::<Vec<u32>>();
    let t = if skip_last {
        t[0..t.len() - 1].to_vec()
    } else {
        t
    };
    t.into_iter().zip(contigous_list).all(|(a, &b)| a == b)
}

fn create_combinations(s0: &str, unknown_indices: &[usize], contigous_list: &[u32]) -> Vec<String> {
    let mut combinations = vec![];
    let mut todo = VecDeque::from([(s0.to_string(), unknown_indices)]);
    while let Some((s, unknown_indices)) = todo.pop_front() {
        if unknown_indices.is_empty() {
            combinations.push(s);
            continue;
        }
        let i = unknown_indices[0];
        let mut s1 = s.to_string();
        s1.replace_range(i..=i, "#");
        if verify_arrangement_partial(&s1, contigous_list) {
            todo.push_back((s1, &unknown_indices[1..]));
        }

        let mut s2 = s.to_string();
        s2.replace_range(i..=i, ".");
        if verify_arrangement_partial(&s2, contigous_list) {
            todo.push_back((s2, &unknown_indices[1..]));
        }
    }
    combinations
}

fn count_arrangements(s: &str) -> u32 {
    let t = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(t.len(), 2);
    let record = t[0];
    let contigous_list = t[1]
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let unknown_indices = record
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    create_combinations(record, &unknown_indices, &contigous_list).len() as u32
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let r1 = lines
        .into_iter()
        .map(|l| count_arrangements(&l))
        .sum::<u32>();
    println!("{}", r1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#.#.###", &[1,1,3], true; "verify_arrangement")]
    #[test_case(".#...#....###.", &[1,1,3], true)]
    #[test_case(".#.###.#.######", &[1,3,1,6], true)]
    #[test_case("####.#...#...", &[4,1,1], true)]
    #[test_case("#....######..#####.", &[1,6,5], true)]
    #[test_case(".###.##....#", &[3,2,1], true)]
    #[test_case("#.#.###", &[1,2,3], false; "verify_arrangement2")]
    #[test_case(".#...#....###.", &[2,1,3], false)]
    #[test_case(".#.###.#.######", &[2,3,1,6], false)]
    #[test_case("####.#...#...", &[4,2,1], false)]
    #[test_case("#....######..#####.", &[1,7,5], false)]
    #[test_case(".###.##....#", &[3,2,2], false)]
    fn test_verify_arrangement(s: &str, contigous_list: &[u32], expected: bool) {
        let result = verify_arrangement(s, contigous_list);
        assert_eq!(expected, result);
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 4)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 1)]
    #[test_case("????.######..#####. 1,6,5", 4)]
    #[test_case("?###???????? 3,2,1", 10)]
    fn test_count_arrangements(s: &str, expected: u32) {
        let result = count_arrangements(s);
        assert_eq!(expected, result);
    }
}
