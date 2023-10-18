use std::io::{self, BufRead};

enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Program {
    instructions: Vec<Instruction>,
    acc: i32,
    idx: usize,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            acc: 0,
            idx: 0,
        }
    }

    pub fn execute(&mut self) {
        match self.instructions[self.idx] {
            Instruction::Acc(arg) => {
                self.acc += arg;
                self.idx += 1;
            }
            Instruction::Jmp(arg) => {
                self.idx = (self.idx as i32 + arg) as usize;
            }
            Instruction::Nop(_) => {
                self.idx += 1;
            }
        }
    }

    pub fn detect_loop(&mut self) -> bool {
        let mut visited = vec![false; self.instructions.len()];
        while self.idx < self.instructions.len() {
            if visited[self.idx] {
                return true;
            }
            visited[self.idx] = true;
            self.execute();
        }
        false
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let instructions = lines
        .iter()
        .map(|l| {
            let mut tokens = l.split(' ');
            let op = tokens.next().unwrap();
            let arg = tokens.next().unwrap().parse::<i32>().unwrap();
            match op {
                "acc" => Instruction::Acc(arg),
                "jmp" => Instruction::Jmp(arg),
                "nop" => Instruction::Nop(arg),
                _ => panic!("invalid instruction"),
            }
        })
        .collect::<Vec<_>>();

    let mut program = Program::new(instructions);
    program.detect_loop();
    println!("{}", program.acc);
}
