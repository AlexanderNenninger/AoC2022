use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, str::FromStr};
///////////////////////////////////////////////////////////////////////////////
type ErasedError = Box<dyn Error + Send + Sync + 'static>;
fn priority(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        return c as u64 - 'a' as u64 + 1;
    }
    if c.is_ascii_uppercase() {
        return c as u64 - 'A' as u64 + 27;
    }
    0
}

fn score_shares<const N: usize>(input: [&str; N]) -> u64 {
    let mut all_present = u64::MAX;
    for i in 0..N {
        let mut counts = 0u64;
        for c in input[i].chars() {
            let nbits = priority(c);
            counts |= 1 << nbits;
        }
        all_present &= counts;
    }
    all_present.trailing_zeros() as u64
}

fn part_1(input: &str) -> u64 {
    let mut out = 0;
    for line in input.lines() {
        out += score_shares([&line[..line.len() / 2], &line[line.len() / 2..]])
    }
    out
}

fn part_2(input: &str) -> u64 {
    let mut out = 0;
    for arr in input.lines().array_chunks::<3>() {
        out += score_shares(arr)
    }
    out
}

pub fn solve() -> SolutionPair {
    const INPUT_FILE: &str = "input/day03.txt";
    let input = read_to_string(INPUT_FILE).expect("ERROR: Input could not be read.");
    let sol1: u64 = part_1(&input.trim());
    let sol2: u64 = part_2(&input.trim());

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_score_shares() {
    let input = ["ah", "hc"];
    let res = score_shares(input);
    assert_eq!(res, 8);

    let input = ["aA", "hA"];
    let res = score_shares(input);
    assert_eq!(res, 27);
}

#[test]
fn test_priority() {
    let c = 'a';
    let prio = priority(c);
    assert_eq!(prio, 1);
    let c = 'A';
    let prio = priority(c);
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
