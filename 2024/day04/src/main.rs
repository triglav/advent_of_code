use std::io;

fn check_xmas(input: &[String]) -> usize {
    let width = input[0].len() as isize;
    let height = input.len() as isize;

    let check_letter = |x: isize, y: isize, letter: char| -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        if x >= width || y >= height {
            return false;
        }
        input[y as usize].chars().nth(x as usize).unwrap() == letter
    };

    let check_xmas_detail = |x: isize, y: isize| -> usize {
        if !check_letter(x, y, 'X') {
            return 0;
        }
        [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .into_iter()
        .filter(|c| {
            check_letter(x + c.0, y + c.1, 'M')
                && check_letter(x + c.0 * 2, y + c.1 * 2, 'A')
                && check_letter(x + c.0 * 3, y + c.1 * 3, 'S')
        })
        .count()
    };

    (0..height)
        .flat_map(|y| (0..width).map(move |x| check_xmas_detail(x, y)))
        .sum()
}

fn main() {
    let input = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let r1 = check_xmas(&input);
    println!("{}", r1);
}
