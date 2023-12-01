use std::io;

fn main() {
    let r1 = io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let c1 = l
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let c2 = l
                .chars()
                .rfind(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            c1 * 10 + c2
        })
        .sum::<u32>();
    println!("{}", r1);
}
