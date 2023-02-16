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
struct Cpu<'a> {
    current_instruction: Option<&'a Instruction>,
    remaining_cycles: i64,
    register: i64,
    cycles: u64,
}

impl<'a> Cpu<'a> {
    fn load_instruction(&mut self, instrunction: &'a Instruction) {
        debug_assert_eq!(self.remaining_cycles, 0);
        self.remaining_cycles = instrunction.cycle_count() as i64;
        self.current_instruction = Some(instrunction);
    }

    fn finish_instruction(&mut self) {
        debug_assert_eq!(self.remaining_cycles, 0);
        match self.current_instruction {
            Some(instruction) => match instruction {
                Instruction::Addx(i) => self.register += i,
                Instruction::Noop => (),
            },
            None => (),
        }
        self.current_instruction = None;
    }

    fn execute_instructions(&mut self, instructions: &'a [Instruction]) {
        let mut instructions = instructions.iter();
        loop {
            self.cycles += 1;
            if self.current_instruction.is_none() {
                // Load new instruction
                if let Some(new_instruction) = instructions.next() {
                    self.load_instruction(new_instruction)
                } else {
                    break;
                }
            }
            if self.remaining_cycles == 0 {
                self.finish_instruction()
            } else {
                self.remaining_cycles -= 1
            }
        }
    }
}

fn part_1(_instructions: &[Instruction]) -> i64 {
    todo!()
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    const INPUT: &str = include_str!("../../input/day10.txt");
    let instructions: Vec<Instruction> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let sol1: i64 = part_1(&instructions);
    let sol2: u64 = 0;

    (Solution::I64(sol1), Solution::U64(sol2))
}

#[test]
fn test_parse_instructions() {
    let input = "noop\naddx 3\naddx -5";
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    dbg!(instructions);
}

#[test]
fn test_execute() {
    let input = "noop\naddx 3\naddx -5";
    let instructions: Vec<Instruction> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut cpu = Cpu {
        current_instruction: None,
        remaining_cycles: 0,
        register: 1,
        cycles: 0,
    };
    cpu.execute_instructions(&instructions);
    assert_eq!(cpu.register, -1);
}
