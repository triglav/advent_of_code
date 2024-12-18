use std::{collections::VecDeque, io};

fn parse_register(s: &str) -> u64 {
    s.split_whitespace().last().unwrap().parse().unwrap()
}

#[derive(Clone, Debug)]
struct State {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    instruction_pointer: usize,
    output: String,
}

impl State {
    fn evaluate_combo_operand(&self, operand: u64) -> u32 {
        (match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand: {}", operand),
        }) as u32
    }

    fn new() -> Self {
        Self {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: String::new(),
        }
    }

    fn add_output(&self, output: u64) -> Self {
        let new_output = if self.output.is_empty() {
            output.to_string()
        } else {
            format!("{},{}", self.output, output)
        };
        Self {
            output: new_output,
            ..*self
        }
    }

    fn advance_pointer(&self, offset: usize) -> Self {
        Self {
            instruction_pointer: self.instruction_pointer + offset,
            output: self.output.clone(),
            ..*self
        }
    }

    fn with_pointer(&self, pos: usize) -> Self {
        Self {
            instruction_pointer: pos,
            output: self.output.clone(),
            ..*self
        }
    }

    fn with_register_a(&self, register_a: u64) -> Self {
        Self {
            register_a,
            output: self.output.clone(),
            ..*self
        }
    }

    fn with_register_b(&self, register_b: u64) -> Self {
        Self {
            register_b,
            output: self.output.clone(),
            ..*self
        }
    }

    fn with_register_c(&self, register_c: u64) -> Self {
        Self {
            register_c,
            output: self.output.clone(),
            ..*self
        }
    }

    fn evaluate_instruction(&self, opcode: u64, operand: u64) -> Self {
        match opcode {
            // adv - division
            0 => {
                let numerator = self.register_a;
                let denominator = u64::pow(2, self.evaluate_combo_operand(operand));
                let result = numerator / denominator;
                self.with_register_a(result).advance_pointer(2)
            }
            // bxl - bitwise xor
            1 => {
                let result = self.register_b ^ operand;
                self.with_register_b(result).advance_pointer(2)
            }
            // bst - modulo 8
            2 => {
                let result = self.evaluate_combo_operand(operand) % 8;
                self.with_register_b(result as u64).advance_pointer(2)
            }
            // jnz - jump if non-zero
            3 => {
                if self.register_a == 0 {
                    self.advance_pointer(2)
                } else {
                    self.with_pointer(operand as usize)
                }
            }
            // bxc - bitwise xor of b and c
            4 => {
                let result = self.register_b ^ self.register_c;
                self.with_register_b(result).advance_pointer(2)
            }
            // out - output operand modulo 8
            5 => {
                let result = self.evaluate_combo_operand(operand) % 8;
                self.add_output(result as u64).advance_pointer(2)
            }
            // bdv - division
            6 => {
                let numerator = self.register_a;
                let denominator = u64::pow(2, self.evaluate_combo_operand(operand));
                let result = numerator / denominator;
                self.with_register_b(result).advance_pointer(2)
            }
            // cdv - division
            7 => {
                let numerator = self.register_a;
                let denominator = u64::pow(2, self.evaluate_combo_operand(operand));
                let result = numerator / denominator;
                self.with_register_c(result).advance_pointer(2)
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}

fn solve(s0: State, program: &[u64]) -> State {
    let mut s = s0;
    while s.instruction_pointer < program.len() {
        let opcode = program[s.instruction_pointer];
        let operand = program[s.instruction_pointer + 1];
        s = s.evaluate_instruction(opcode, operand);
    }
    s
}

fn solve2(program: &[u64]) -> Option<u64> {
    let mut to_visit = VecDeque::from([(program.len(), 0)]);

    while let Some((pos, a)) = to_visit.pop_front() {
        for i in 0..8 {
            let n_a = a * 8 + i;
            let s0 = State::new().with_register_a(n_a);
            let s2 = solve(s0, program);
            let o = s2
                .output
                .split(',')
                .map(|a| a.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if o == program[pos - 1..] {
                to_visit.push_back((pos - 1, n_a));
                if o.len() == program.len() {
                    return Some(n_a);
                }
            }
        }
    }
    None
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let register_a = parse_register(lines[0].as_str());
    let register_b = parse_register(lines[1].as_str());
    let register_c = parse_register(lines[2].as_str());
    let program0 = lines[4].split_whitespace().last().unwrap();
    let program = program0
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let s0 = State::new()
        .with_register_a(register_a)
        .with_register_b(register_b)
        .with_register_c(register_c);
    let s = solve(s0, &program);
    let r1 = s.output;
    println!("{}", r1);

    let r2 = solve2(&program).unwrap();
    println!("{}", r2);
    // verify
    let s2 = solve(State::new().with_register_a(r2), &program);
    assert_eq!(s2.output, program0);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn adv_should_perform_division() {
        let s0 = State::new().with_register_a(10);
        let s2 = s0.evaluate_instruction(0, 2);
        assert_eq!(s2.register_a, 2);
        assert_eq!(s2.instruction_pointer, 2);
    }

    #[test]
    fn adv_should_perform_with_register() {
        let s0 = State::new()
            .with_register_a(10)
            .with_register_b(1)
            .with_pointer(42);
        let s2 = s0.evaluate_instruction(0, 5);
        assert_eq!(s2.register_a, 5);
        assert_eq!(s2.instruction_pointer, 44);
    }

    #[test]
    fn bxl_should_bitwise_xor() {
        let s0 = State::new().with_register_a(10).with_register_b(5);
        let s2 = s0.evaluate_instruction(1, 3);
        assert_eq!(s2.register_b, 6);
        assert_eq!(s2.instruction_pointer, 2);
    }

    #[test]
    fn bst_should_modulo_8() {
        let s0 = State::new().with_register_a(10).with_register_b(5);
        let s2 = s0.evaluate_instruction(2, 4);
        assert_eq!(s2.register_b, 2);
        assert_eq!(s2.instruction_pointer, 2);
    }

    #[test]
    fn jnz_does_nothing_if_register_a_is_zero() {
        let s0 = State::new()
            .with_register_b(5)
            .with_register_c(3)
            .with_pointer(10);
        let s2 = s0.evaluate_instruction(3, 4);
        assert_eq!(s2.instruction_pointer, 12);
    }

    #[test]
    fn jnz_jumps_if_register_a_is_non_zero() {
        let s0 = State::new()
            .with_register_a(1)
            .with_register_b(5)
            .with_register_c(3)
            .with_pointer(10);
        let s2 = s0.evaluate_instruction(3, 4);
        assert_eq!(s2.instruction_pointer, 4);
    }

    #[test]
    fn bxc_bitwise_xor_of_b_and_c() {
        let s0 = State::new()
            .with_register_a(10)
            .with_register_b(2)
            .with_register_c(10);
        let s2 = s0.evaluate_instruction(4, 42);
        assert_eq!(s2.register_b, 8);
        assert_eq!(s2.instruction_pointer, 2);
    }

    #[test]
    fn bdv_should_perform_division() {
        let s0 = State::new().with_register_a(10);
        let s2 = s0.evaluate_instruction(6, 2);
        assert_eq!(s2.register_b, 2);
        assert_eq!(s2.instruction_pointer, 2);
    }

    #[test]
    fn cdv_should_perform_division() {
        let s0 = State::new().with_register_a(10);
        let s2 = s0.evaluate_instruction(7, 2);
        assert_eq!(s2.register_c, 2);
        assert_eq!(s2.instruction_pointer, 2);
    }
}
