use std::io;

fn hash(s: &str) -> u8 {
    s.chars()
        .map(|c| c as u8)
        .fold(0u32, |a, c| ((a + c as u32) * 17) % 256) as u8
}

fn main() {
    let line = io::stdin().lines().map(|l| l.unwrap()).next().unwrap();
    let steps = line.split(',');

    let r1 = steps.map(|s| hash(s) as u32).sum::<u32>();
    println!("{}", r1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("HASH", 52)]
    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("qp=3", 97)]
    #[test_case("cm=2", 47)]
    #[test_case("qp-", 14)]
    #[test_case("pc=4", 180)]
    #[test_case("ot=9", 9)]
    #[test_case("ab=5", 197)]
    #[test_case("pc-", 48)]
    #[test_case("pc=6", 214)]
    #[test_case("ot=7", 231)]
    fn test_hash(s: &str, expected: u8) {
        let result = hash(s);
        assert_eq!(expected, result);
    }
}
