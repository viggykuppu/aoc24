use std::collections::HashSet;

use aocd::*;
use regex::Regex;

#[aocd(2024, 17)]
pub fn one() {
    let input = input!();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut computer = Computer {
        registers: [0, 0, 0],
        instruction_pointer: 0,
        instructions: Vec::new(),
    };
    input.lines().for_each(|line| {
        let x = num_regex.find(line);
        if line.contains('A') {
            computer.registers[0] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains('B') {
            computer.registers[1] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains('C') {
            computer.registers[2] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains("Program") {
            let instructions: Vec<_> = line
                .split(": ")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            computer.instructions = instructions;
        }
    });
    let output = computer.run();
    let output = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");
    submit!(1, output);
}

#[aocd(2024, 17)]
pub fn two() {
    let input = input!();
    let num_regex = Regex::new(r"\d+").unwrap();
    let mut computer = Computer {
        registers: [0, 0, 0],
        instruction_pointer: 0,
        instructions: Vec::new(),
    };
    input.lines().for_each(|line| {
        let x = num_regex.find(line);
        if line.contains('A') {
            computer.registers[0] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains('B') {
            computer.registers[1] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains('C') {
            computer.registers[2] = x.unwrap().as_str().parse::<u64>().unwrap();
        } else if line.contains("Program") {
            let instructions: Vec<_> = line
                .split(": ")
                .collect::<Vec<_>>()
                .get(1)
                .unwrap()
                .split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect();
            computer.instructions = instructions;
        }
    });
    let mut x = foo(0, computer, &mut HashSet::new());
    x.sort();
    let solution = x[0];
    submit!(2, solution);
}

fn foo(x: u64, computer: Computer, tried: &mut HashSet<u64>) -> Vec<u64> {
    let mut computer = computer.clone();
    computer.registers = [x, 0, 0];
    computer.instruction_pointer = 0;
    let output = computer.run();
    if output.len() > computer.instructions.len() {
        return vec![];
    }
    if output == computer.instructions {
        return vec![x];
    }
    let mut potentials = Vec::new();
    for k in 1..=64 {
        let j = k % 8;
        let p = (k & 0b111000) << 3;
        let test_number = (x * 8 & !(0b111000000)) + j + p;
        if tried.contains(&test_number) {
            continue;
        }
        let mut computer = computer.clone();
        computer.registers = [test_number, 0, 0];
        computer.instruction_pointer = 0;
        let output = computer.run();
        if output.len() > computer.instructions.len() {
            return vec![];
        }
        if output == computer.instructions[computer.instructions.len() - output.len()..] {
            if output.len() == computer.instructions.len() {
                return vec![test_number];
            }
            tried.insert(test_number);
            let y = foo(test_number, computer, tried);
            potentials = [potentials, y].concat();
        }
    }
    return potentials;
}

#[derive(Debug, Clone)]
struct Computer {
    registers: [u64; 3],
    instruction_pointer: usize,
    instructions: Vec<u8>,
}

impl Computer {
    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();
        while self.instruction_pointer < self.instructions.len() {
            let opcode: u8 = self.instructions[self.instruction_pointer];
            let literal_operand = self.instructions[self.instruction_pointer + 1] as u64;
            let mut jumped = false;
            match opcode {
                0 => {
                    let combo_operand_value = self.get_combo_operand_value(literal_operand);
                    let numerator = self.registers[0];
                    let denominator = 2_u64.pow(combo_operand_value as u32);
                    self.registers[0] = numerator / denominator;
                }
                1 => {
                    self.registers[1] = self.registers[1] ^ literal_operand;
                }
                2 => {
                    let combo_operand_value = self.get_combo_operand_value(literal_operand);
                    self.registers[1] = combo_operand_value % 8;
                }
                3 => {
                    if self.registers[0] != 0 {
                        self.instruction_pointer = literal_operand as usize;
                        jumped = true;
                    }
                }
                4 => {
                    self.registers[1] = self.registers[1] ^ self.registers[2];
                }
                5 => {
                    let combo_operand_value = self.get_combo_operand_value(literal_operand);
                    let x = (combo_operand_value % 8) as u8;
                    output.push(x);
                }
                6 => {
                    let combo_operand_value = self.get_combo_operand_value(literal_operand);
                    let numerator = self.registers[0];
                    let denominator = 2_u64.pow(combo_operand_value as u32);
                    self.registers[1] = numerator / denominator;
                }
                7 => {
                    let combo_operand_value = self.get_combo_operand_value(literal_operand);
                    let numerator = self.registers[0];
                    let denominator = 2_u64.pow(combo_operand_value as u32);
                    self.registers[2] = numerator / denominator;
                }
                _ => {
                    println!("fucked up");
                }
            }
            if !jumped {
                self.instruction_pointer += 2;
            }
        }
        return output;
    }

    fn get_combo_operand_value(&self, operand: u64) -> u64 {
        if (0..=3).contains(&operand) {
            return operand as u64;
        } else {
            return self.registers[(operand - 4) as usize] as u64;
        }
    }
}
