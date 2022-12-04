use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, num::ParseIntError, str::FromStr};

const TEST_INPUT: &str = include_str!("../../input/day01_test.txt");
const INPUT: &str = include_str!("../../input/day01.txt");
///////////////////////////////////////////////////////////////////////////////

struct Elf {
    meals: Vec<u64>,
}

impl Elf {
    fn total_calories(&self) -> u64 {
        self.meals.iter().sum()
    }
}

impl FromStr for Elf {
    type Err = Box<dyn Error + Send + Sync + 'static>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let meals = s
            .split_whitespace()
            .map(|c| c.parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;
        Ok(Elf { meals })
    }
}

fn part_1(input: &str) -> u64 {
    let elfs: Vec<Elf> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    elfs.iter().map(|elf| elf.total_calories()).max().unwrap()
}

fn part_2(input: &str) -> u64 {
    let elfs: Vec<Elf> = input.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut calories: Vec<u64> = elfs.into_iter().map(|elf| elf.total_calories()).collect();
    calories.sort();
    calories[calories.len() - 3..].iter().sum()
}

pub fn solve() -> SolutionPair {
    let sol1: u64 = part_1(&INPUT);
    let sol2: u64 = part_2(&INPUT);

    (Solution::U64(sol1), Solution::U64(sol2))
}
