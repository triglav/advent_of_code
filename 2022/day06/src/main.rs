use itertools::izip;
use std::io;

type Sequence = (char, char, char, char);

fn is_start_of_packet_marker((a, b, c, d): Sequence) -> bool {
    a != b && a != c && a != d && b != c && b != d && c != d
}

fn main() {
    let buf = io::stdin().lines().next().unwrap().unwrap();

    let r1 = izip!(
        buf.chars(),
        buf.chars().skip(1),
        buf.chars().skip(2),
        buf.chars().skip(3)
    )
    .position(is_start_of_packet_marker)
    .unwrap() + 4;
    println!("{}", r1);
}
