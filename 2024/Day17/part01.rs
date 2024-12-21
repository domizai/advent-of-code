// rustc part01.rs -Oo main && ./main

#![allow(non_snake_case, unused_variables)]

#[cfg(windows)]
const NEWLINE: &'static str = "\r\n";
#[cfg(not(windows))]
const NEWLINE: &'static str = "\n";

#[derive(Debug)]
struct Registers {
    A: usize,
    B: usize,
    C: usize,
    PC: usize,
}

impl Registers {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.A,
            5 => self.B,
            6 => self.C,
            _ => panic!("Invalid combo operand {}", operand),
        }
    }
}

#[derive(Debug)]
struct Flags {
    INC_PC: bool,
    OUT: Option<usize>,
}

#[derive(Debug)]
struct CPU {
    reg: Registers,
    flags: Flags,
}

impl CPU {
    fn exec(&mut self, mem: &Vec<usize>) -> String {
        let mut outputs: Vec<usize> = Vec::new();

        while self.reg.PC < mem.len() - 1 {
            let opcode = mem[self.reg.PC];
            let operand = mem[self.reg.PC + 1];
            self.control_unit(opcode, operand);
            self.reg.PC += 2 * self.flags.INC_PC as usize;
            self.flags.INC_PC = true;
            if let Some(output) = self.flags.OUT {
                outputs.push(output);
                self.flags.OUT = None;
            }
        }
        outputs.iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn control_unit(&mut self, opcode: usize, operand: usize) {
        // instruction set
        match opcode {
            /* adv */ 0 => self.reg.A >>= self.reg.combo(operand),
            /* bxl */ 1 => self.reg.B ^= operand,
            /* bst */ 2 => self.reg.B = self.reg.combo(operand) & 0x7,
            /* jnz */ 3 => { if self.reg.A == 0 { return; } self.reg.PC = operand; self.flags.INC_PC = false; },
            /* bxc */ 4 => self.reg.B ^= self.reg.C,
            /* out */ 5 => self.flags.OUT = Some(self.reg.combo(operand) & 0x7),
            /* bdv */ 6 => self.reg.B = self.reg.A >> self.reg.combo(operand),
            /* cdv */ 7 => self.reg.C = self.reg.A >> self.reg.combo(operand),
            _ => panic!("Invalid opcode {}", opcode),
        }
    }
}

fn main() {
    let input = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    let input = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<String> = input.trim()
        .split(format!("{}{}", NEWLINE, NEWLINE).as_str())
        .map(|s| s.to_string())
        .collect();

    let mem: Vec<usize> = input[1]
        .replace("Program: ", "")
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();
        
    let reg: Vec<usize> = input[0].lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse().unwrap())
        .collect();

    let mut cpu = CPU {
        reg: Registers { A: reg[0], B: reg[1], C: reg[2], PC: 0 },
        flags: Flags { INC_PC: true, OUT: None },
    };

    let output = cpu.exec(&mem);
    println!("{}", output);
}