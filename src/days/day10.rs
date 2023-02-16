use std::str::FromStr;

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl Instruction {
    fn cycle_count(&self) -> u64 {
        match self {
            Instruction::Addx(_) => 2,
            Instruction::Noop => 1,
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines().map(|l| l.parse().unwrap()).rev().collect()
}

impl FromStr for Instruction {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        match parts.next().ok_or("ERROR: Empty input.".to_string())? {
            "noop" => return Ok(Instruction::Noop),
            "addx" => {
                let arg = parts
                    .next()
                    .ok_or("ERROR: Addx without arguemnt.".to_string())?
                    .parse()?;
                return Ok(Instruction::Addx(arg));
            }
            _ => Err("ERROR: Unknown instruction.".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct CPU {
    instruction_stack: Vec<Instruction>,
    current_instruction: Option<Instruction>,
    remaining_cycles: i64,
    register: i64,
    cycles: u64,
}

impl CPU {
    fn new(instructions: Vec<Instruction>) -> Self {
        CPU {
            instruction_stack: instructions,
            current_instruction: None,
            remaining_cycles: 0,
            register: 1,
            cycles: 0,
        }
    }

    fn load_instruction(&mut self) {
        debug_assert_eq!(self.remaining_cycles, 0);
        let instruction = self.instruction_stack.pop();
        if let Some(instruction) = &instruction {
            self.remaining_cycles = instruction.cycle_count() as i64
        }
        self.current_instruction = instruction;
    }

    fn finish_instruction(&mut self) {
        debug_assert_eq!(self.remaining_cycles, 0);
        if let Some(instruction) = &self.current_instruction {
            match instruction {
                Instruction::Addx(i) => self.register += i,
                Instruction::Noop => (),
            }
        }
        self.current_instruction = None;
    }

    fn tick(&mut self) {
        if self.remaining_cycles == 0 {
            self.finish_instruction();
            self.load_instruction();
        }
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
        }
        self.cycles += 1;
    }
}

fn part_1(instructions: Vec<Instruction>) -> i64 {
    let mut cpu = CPU::new(instructions);
    cpu.load_instruction();
    let mut signal_strength = 0;
    while cpu.instruction_stack.len() > 0 || cpu.current_instruction.is_some() {
        cpu.tick();
        if cpu.cycles % 40 == 20 {
            signal_strength += cpu.cycles as i64 * cpu.register;
        }
    }
    signal_strength
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day10.txt");
    let instructions: Vec<Instruction> = INPUT.lines().map(|l| l.parse().unwrap()).rev().collect();
    let sol1: i64 = part_1(instructions);
    let sol2: u64 = 0;

    (Solution::I64(sol1), Solution::U64(sol2))
}

mod tests {
    use super::{parse_instructions, part_1, Instruction, CPU};

    #[test]
    fn test_parse_instructions() {
        let input = "noop\naddx 3\naddx -5";
        let instructions: Vec<Instruction> =
            input.lines().map(|l| l.parse().unwrap()).rev().collect();
        dbg!(instructions);
    }

    #[test]
    fn test_tick() {
        let input = "noop\naddx 3\naddx -5";
        let instructions: Vec<Instruction> =
            input.lines().map(|l| l.parse().unwrap()).rev().collect();
        let mut cpu = CPU {
            instruction_stack: instructions,
            current_instruction: None,
            remaining_cycles: 0,
            register: 1,
            cycles: 0,
        };

        let register_values = [1, 1, 1, 4, 4, -1];
        while cpu.instruction_stack.len() > 0 || cpu.current_instruction.is_some() {
            cpu.tick();
            assert_eq!(cpu.register, register_values[cpu.cycles as usize - 1])
        }
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../../input/day10_test.txt");
        let instructions = parse_instructions(input);
        let res = part_1(instructions);
        assert_eq!(res, 13140)
    }
}
