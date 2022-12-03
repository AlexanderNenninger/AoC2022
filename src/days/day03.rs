use itertools::Itertools;

use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, str::FromStr};
///////////////////////////////////////////////////////////////////////////////
type ErasedError = Box<dyn Error + Send + Sync + 'static>;
fn priority(c: char) -> Result<u64, ErasedError> {
    if c.is_ascii_lowercase() {
        return Ok(c as u64 - 'a' as u64 + 1);
    }
    if c.is_ascii_uppercase() {
        return Ok(c as u64 - 'A' as u64 + 27);
    }
    return Err("ERROR: character invalid.".into());
}

struct Rucksack {
    compartment_1: Vec<u64>,
    compartment_2: Vec<u64>,
}

impl FromStr for Rucksack {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n_items = s.len();
        if n_items % 2 != 0 {
            return Err("ERROR: Input has uneven number of items.".into());
        }
        let items: Vec<u64> = s
            .chars()
            .into_iter()
            .map(priority)
            .collect::<Result<_, ErasedError>>()?;
        Ok(Rucksack {
            compartment_1: items[..n_items / 2].to_vec(),
            compartment_2: items[n_items / 2..].to_vec(),
        })
    }
}
impl Rucksack {
    fn score_duplicated(&self) -> u64 {
        let mut counts_1 = [0 as u64; 52];
        let mut counts_2 = [0 as u64; 52];
        for item in self.compartment_1.iter() {
            counts_1[*item as usize - 1] += 1
        }
        for item in self.compartment_2.iter() {
            counts_2[*item as usize - 1] += 1
        }

        counts_1
            .into_iter()
            .zip(counts_2)
            .enumerate()
            .map(|(idx, (c1, c2))| if c1 > 0 && c2 > 0 { idx + 1 } else { 0 })
            .sum::<usize>() as u64
    }
}

fn part_1(input: &str) -> u64 {
    let rucksacks: Vec<Rucksack> = input.lines().map(|l| l.parse().unwrap()).collect();
    rucksacks.iter().map(|r| r.score_duplicated()).sum()
}

struct Group {
    elf_1: Vec<u64>,
    elf_2: Vec<u64>,
    elf_3: Vec<u64>,
}

impl FromStr for Group {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err("ERROR: Input has uneven number of items.".into());
        }
        if s.lines().count() != 3 {
            return Err("ERROR: Number of lines in Input does not equal 3.".into());
        }
        let mut lines = s.lines();
        Ok(Group {
            elf_1: lines
                .next()
                .unwrap()
                .chars()
                .map(priority)
                .collect::<Result<_, ErasedError>>()?,
            elf_2: lines
                .next()
                .unwrap()
                .chars()
                .map(priority)
                .collect::<Result<_, ErasedError>>()?,
            elf_3: lines
                .next()
                .unwrap()
                .chars()
                .map(priority)
                .collect::<Result<_, ErasedError>>()?,
        })
    }
}

impl Group {
    fn label_group(&self) -> u64 {
        let mut counts_1 = [0; 52];
        let mut counts_2 = [0; 52];
        let mut counts_3 = [0; 52];

        for item in self.elf_1.iter() {
            counts_1[*item as usize - 1] += 1
        }
        for item in self.elf_2.iter() {
            counts_2[*item as usize - 1] += 1
        }
        for item in self.elf_3.iter() {
            counts_3[*item as usize - 1] += 1
        }

        for i in 0..52 {
            if counts_1[i] > 0 && counts_2[i] > 0 && counts_3[i] > 0 {
                return i as u64 + 1;
            }
        }
        0
    }
}

fn part_2(input: &str) -> u64 {
    let n_groups = input.lines().count() / 3;
    let mut groups: Vec<Group> = Vec::with_capacity(n_groups);

    for mut g in &input.lines().chunks(3) {
        let group_input = g.join("\n");
        groups.push(group_input.parse().expect("ERROR: Could not parse input."));
    }
    return groups.iter().map(|g| g.label_group()).sum();
}

pub fn solve() -> SolutionPair {
    const INPUT_FILE: &str = "input/day03.txt";
    let input = read_to_string(INPUT_FILE).expect("ERROR: Input could not be read.");
    let sol1: u64 = part_1(&input.trim());
    let sol2: u64 = part_2(&input.trim());

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_priority() {
    let c = 'a';
    let prio = priority(c).unwrap();
    assert_eq!(prio, 1);
    let c = 'A';
    let prio = priority(c).unwrap();
    assert_eq!(prio, 27);
}

#[test]
fn test_part_1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    let sol = part_1(input);
    assert_eq!(sol, 157)
}

#[test]
fn test_part_2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    let sol = part_2(input);
    assert_eq!(sol, 70)
}
