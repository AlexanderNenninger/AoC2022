use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, num::ParseIntError, str::FromStr};

const TEST_INPUT: &str = "input/day01_test.txt";
const INPUT: &str = "input/day01.txt";
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

fn part_1() -> u64 {
    let data = read_to_string(INPUT).unwrap();
    let elfs: Vec<Elf> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();
    elfs.iter().map(|elf| elf.total_calories()).max().unwrap()
}

fn part_2() -> u64 {
    let data = read_to_string(INPUT).unwrap();
    let elfs: Vec<Elf> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();
    let mut calories: Vec<u64> = elfs.into_iter().map(|elf| elf.total_calories()).collect();
    calories.sort();
    calories[calories.len() - 3..].iter().sum()
}

pub fn solve() -> SolutionPair {
    let sol1: u64 = part_1();
    let sol2: u64 = part_2();

    (Solution::U64(sol1), Solution::U64(sol2))
}
