use crate::{etc::ErasedError, Solution, SolutionPair};
use std::str::FromStr;

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq)]
struct Move {
    origin: usize,
    destination: usize,
    count: usize,
}

impl FromStr for Move {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .take(3);

        let count = data.next().ok_or("ERROR: Invalid input.".to_string())?;
        let origin = data.next().ok_or("ERROR: Invalid input.".to_string())?;
        let destination = data.next().ok_or("ERROR: Invalid input.".to_string())?;

        Ok(Move {
            origin,
            destination,
            count,
        })
    }
}

impl Move {
    fn apply<const N: usize>(&self, stacks: &mut Stacks<N>) -> Result<(), ErasedError> {
        for _ in 0..self.count {
            let item = stacks.stacks[self.origin - 1]
                .pop()
                .ok_or("ERROR: Stack empty.".to_string())?;
            stacks.stacks[self.destination - 1].push(item);
        }
        Ok(())
    }
    fn apply_2<const N: usize>(&self, stacks: &mut Stacks<N>) -> Result<(), ErasedError> {
        let mut buffer = vec![];
        for _ in 0..self.count {
            let item = stacks.stacks[self.origin - 1]
                .pop()
                .ok_or("ERROR: Stack empty.".to_string())?;
            buffer.push(item);
        }
        for _ in 0..buffer.len() {
            let item = buffer.pop().expect("ERROR: Buffer empty");
            stacks.stacks[self.destination - 1].push(item);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stacks<const N: usize> {
    stacks: [Vec<char>; N],
}

impl<const N: usize> Stacks<N> {
    fn get_message(&self) -> String {
        let mut out: String = "".into();
        for i in 0..N {
            if let Some(c) = self.stacks[i].last() {
                out.push(*c)
            }
        }
        out
    }
}

impl<const N: usize> FromStr for Stacks<N> {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const INIT: Vec<char> = Vec::new();
        let mut stacks: [Vec<char>; N] = [INIT; N];
        let mut lines = s.lines().rev();
        let index = lines.next().ok_or("ERROR: Empty input.".to_string())?;

        for line in lines {
            let mut stack_idx = 0;
            for (i, index_char) in index.char_indices() {
                let to_push = if let Some(c) = line.as_bytes().get(i) {
                    *c as char
                } else {
                    continue;
                };
                if index_char.is_numeric() {
                    if to_push.is_alphabetic() {
                        stacks[stack_idx].push(to_push);
                    }
                    stack_idx += 1;
                }
            }
        }
        Ok(Stacks { stacks })
    }
}

fn part_1<const N: usize>(input: &str) -> String {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks<N> = stacks_input.parse().unwrap();
    let moves: Vec<Move> = moves_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    for move_ in moves {
        move_.apply(&mut stacks).expect("ERROR: Move failed.");
    }

    return stacks.get_message();
}
fn part_2<const N: usize>(input: &str) -> String {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();
    let mut stacks: Stacks<N> = stacks_input.parse().unwrap();
    let moves: Vec<Move> = moves_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    for move_ in moves {
        move_.apply_2(&mut stacks).expect("ERROR: Move failed.");
    }

    return stacks.get_message();
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day05.txt");
    // Your solution here...
    let sol1: String = part_1::<9>(&INPUT);
    let sol2: String = part_2::<9>(&INPUT);

    (Solution::Str(sol1), Solution::Str(sol2))
}

#[test]
fn test_parse_stacks() {
    const DATA: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ";
    println!("{}", DATA);
    let stacks: Stacks<3> = DATA.parse().unwrap();
    println!("{:?}", stacks);
    assert_eq!(
        stacks,
        Stacks {
            stacks: [vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        }
    )
}

#[test]
fn test_part_1() {
    const INPUT: &str = include_str!("../../input/day05_test.txt");
    let res = part_1::<3>(&INPUT);
    println!("{}", res);
}
