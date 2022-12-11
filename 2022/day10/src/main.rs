use std::io;

enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_instruction(s: &String) -> Instruction {
    let t = s.split(" ").collect::<Vec<&str>>();
    match t[0] {
        "noop" => Instruction::Noop,
        "addx" => Instruction::AddX(t[1].parse::<i32>().unwrap()),
        _ => panic!("Invalid instruction"),
    }
}

struct Cpu {
    cycle: i32,
    reg_x: i32,
}

impl Cpu {
    fn execute(&self, ins: &Instruction) -> Vec<Cpu> {
        match ins {
            Instruction::Noop => vec![Cpu {
                cycle: self.cycle + 1,
                reg_x: self.reg_x,
            }],
            Instruction::AddX(v) => vec![
                Cpu {
                    cycle: self.cycle + 1,
                    reg_x: self.reg_x,
                },
                Cpu {
                    cycle: self.cycle + 2,
                    reg_x: self.reg_x + v,
                },
            ],
        }
    }

    fn signal_strength(&self) -> i32 {
        self.cycle * self.reg_x
    }
}

fn main() {
    let instructions = io::stdin().lines().map(|l| parse_instruction(&l.unwrap()));
    let cycles = instructions.fold(vec![Cpu { cycle: 1, reg_x: 1 }], |mut a, ins| {
        let new_cycles = a.last().unwrap().execute(&ins);
        a.extend(new_cycles);
        a
    });
    let interesting_cycle_numbers = vec![20, 60, 100, 140, 180, 220];
    let interesting_cycles = cycles
        .iter()
        .filter(|c| interesting_cycle_numbers.contains(&c.cycle));
    let r1 = interesting_cycles.map(|c| c.signal_strength()).sum::<i32>();
    println!("{}", r1);

    let width = 40;
    for c in cycles.iter() {
        let drawing_pixel = (c.cycle - 1) % width;
        let pixel_lit = drawing_pixel >= c.reg_x - 1 && drawing_pixel <= c.reg_x + 1;
        print!("{}", if pixel_lit { '#' } else { '.' });
        if c.cycle % width == 0 {
            println!();
        }
    }
}
