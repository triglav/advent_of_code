use std::{collections::HashMap, io};

fn is_long_enough(remaining: usize, contigous_list: &[u32]) -> bool {
    assert!(!contigous_list.is_empty());
    contigous_list.iter().sum::<u32>() as usize + (contigous_list.len() - 1) <= remaining
}

fn check_spring(s: &str, size: usize) -> Option<(String, &str)> {
    let border = s.chars().nth(size);
    let s2 = if border.is_some() {
        &s[(size + 1)..]
    } else {
        &s[s.len()..]
    };
    ((border == Some('.') || border == Some('?') || border.is_none())
        && s.len() >= size
        && s[0..size].chars().all(|c| c == '?' || c == '#'))
    .then_some((
        format!(
            "{}{}",
            "#".repeat(size),
            if border.is_some() { "." } else { "" }
        ),
        s2,
    ))
}

fn create_combinations(s: &str, contigous_list0: &[u32]) -> u64 {
    let mut seen = HashMap::<(usize, usize), u64>::new();
    fn create_combinations_rec(
        history: String,
        s: &str,
        contigous_list: &[u32],
        seen: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        // skip all the dots
        let p = s.chars().position(|c| c != '.').unwrap_or(s.len());
        let s = &s[p..];
        let history = [history, ".".repeat(p)].concat();

        if let Some(c) = seen.get(&(s.len(), contigous_list.len())) {
            return *c;
        }

        if contigous_list.is_empty() {
            let c = if s.chars().all(|c| c == '.' || c == '?') {
                1
            } else {
                0
            };
            seen.insert((s.len(), contigous_list.len()), c);
            return c;
        }

        let first_to_place = contigous_list[0] as usize;
        let mut c = 0;
        if let Some((h, s2)) = check_spring(s, first_to_place) {
            let l2 = &contigous_list[1..];
            let c2 = create_combinations_rec(format!("{}{}", history, h), s2, l2, seen);
            seen.insert((s2.len(), l2.len()), c2);
            c += c2;
        }
        if s.starts_with('?') {
            let s2 = s.strip_prefix('?').unwrap();
            let c2 = if is_long_enough(s2.len(), contigous_list) {
                create_combinations_rec(format!("{}.", history), s2, contigous_list, seen)
            } else {
                0
            };
            seen.insert((s2.len(), contigous_list.len()), c2);
            c += c2;
        }
        c
    }
    create_combinations_rec("".to_string(), s, contigous_list0, &mut seen)
}

fn count_arrangements(s: &str) -> u64 {
    let t = s.split(' ').collect::<Vec<&str>>();
    assert_eq!(t.len(), 2);
    let record = t[0];
    let contigous_list = t[1]
        .split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    create_combinations(record, &contigous_list)
}

fn unfold(s: &str) -> String {
    let t = s.split(' ').collect::<Vec<&str>>();
    let t1 = [t[0]; 5].join("?");
    let t2 = [t[1]; 5].join(",");
    format!("{t1} {t2}")
}

fn main() {
    let lines = io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let r1 = lines.iter().map(|l| count_arrangements(l)).sum::<u64>();
    println!("{}", r1);

    let r2 = lines
        .into_iter()
        .map(|l| count_arrangements(unfold(&l).as_str()))
        .sum::<u64>();
    println!("{}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#", 1, Some(("#".to_string(), "")); "check_spring")]
    #[test_case("#.", 1,   Some(("#.".to_string(), "")); "check_spring 2")]
    #[test_case("##", 2,   Some(("##".to_string(), "")); "check_spring 3")]
    #[test_case("##.", 2,  Some(("##.".to_string(), "")); "check_spring 4")]
    #[test_case("?.##", 1, Some(("#.".to_string(), "##")); "check_spring 5")]
    #[test_case("?#.", 1,  None; "check_spring 6")]
    #[test_case("?#.", 2,  Some(("##.".to_string(), "")); "check_spring 7")]
    #[test_case("###....###", 3,   Some(("###.".to_string(), "...###")); "check_spring 8")]
    #[test_case("###.???..", 3,   Some(("###.".to_string(), "???..")); "check_spring 9")]
    fn test_check_spring(s: &str, size: usize, expected: Option<(String, &str)>) {
        let result = check_spring(s, size);
        assert_eq!(expected, result);
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 4)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 1)]
    #[test_case("????.######..#####. 1,6,5", 4)]
    #[test_case("?###???????? 3,2,1", 10)]
    #[test_case("?.?#?#??#?#????#??.. 9,4", 5)]
    #[test_case("??????###.???.. 1,3", 5)]
    fn test_count_arrangements(s: &str, expected: u64) {
        let result = count_arrangements(s);
        assert_eq!(expected, result);
    }

    #[test_case("???.### 1,1,3", 1)]
    #[test_case(".??..??...?##. 1,1,3", 16384)]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[test_case("????.#...#... 4,1,1", 16)]
    #[test_case("????.######..#####. 1,6,5", 2500)]
    #[test_case("?###???????? 3,2,1", 506250)]
    fn test_unfold_count_arrangements(s: &str, expected: u64) {
        let result = count_arrangements(unfold(s).as_str());
        assert_eq!(
            expected, result,
            "'{s}' expected {expected} but got {result}"
        )
    }

    #[test]
    fn test_unfold() {
        let result = unfold(".# 1");
        let expected = ".#?.#?.#?.#?.# 1,1,1,1,1";
        assert_eq!(expected, result);
    }
}
