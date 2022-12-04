use crate::{Solution, SolutionPair};
use std::{error::Error, fs::read_to_string, str::FromStr};

///////////////////////////////////////////////////////////////////////////////
type ErasedError = Box<dyn Error + Send + Sync + 'static>;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Interval<T> {
    lower_bound: T,
    upper_bound: T,
}

impl<T: PartialOrd> Interval<T> {
    fn new(lower_bound: T, upper_bound: T) -> Result<Self, String> {
        if !(lower_bound <= upper_bound) {
            return Err("ERROR: Lower bound of Interval larger than upper bound.".to_string());
        }
        Ok(Interval {
            lower_bound,
            upper_bound,
        })
    }

    #[inline]
    fn check(&self) -> () {
        if self.lower_bound > self.upper_bound {
            panic!("ERROR: Lower bound of Interval larger than upper bound.")
        }
    }

    fn is_subset(&self, other: &Interval<T>) -> bool {
        // Assumption: self.lower_bound <= self.upper_bound
        self.check();
        other.check();
        if self.lower_bound >= other.lower_bound && self.upper_bound <= other.upper_bound {
            return true;
        }
        false
    }

    fn intersects(&self, other: &Interval<T>) -> bool {
        // Assumption: self.lower_bound <= self.upper_bound
        self.check();
        other.check();
        if self.lower_bound <= other.upper_bound && other.upper_bound <= self.upper_bound {
            return true;
        }
        if other.lower_bound <= self.upper_bound && self.upper_bound <= other.upper_bound {
            return true;
        }
        false
    }
}

impl<T> FromStr for Interval<T>
where
    T: PartialOrd + FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        let lower_bound = parts
            .next()
            .ok_or("ERROR: Empty input.".to_string())?
            .parse()?;
        let upper_bound = parts
            .next()
            .ok_or("ERROR: Empty input.".to_string())?
            .parse()?;
        Ok(Interval::new(lower_bound, upper_bound)?)
    }
}

fn prepare_input(input: &str) -> Vec<(Interval<i64>, Interval<i64>)> {
    let n_lines = input.lines().count();
    let mut intervals = Vec::<(Interval<i64>, Interval<i64>)>::with_capacity(n_lines);
    for line in input.lines() {
        let mut parts = line.split(",");
        intervals.push((
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        ))
    }
    intervals
}

fn part_1<T: PartialOrd>(intervals: &Vec<(Interval<T>, Interval<T>)>) -> u64 {
    intervals
        .iter()
        .map(|(i, j)| (i.is_subset(j) || j.is_subset(i)) as u64)
        .sum()
}

fn part_2<T: PartialOrd>(intervals: &Vec<(Interval<T>, Interval<T>)>) -> u64 {
    intervals
        .iter()
        .map(|(i, j)| (i.intersects(j) || j.intersects(i)) as u64)
        .sum()
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day04.txt");
    let intervals = prepare_input(&INPUT);
    let sol1: u64 = part_1(&intervals);
    let sol2: u64 = part_2(&intervals);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_interval() {
    let interval24: Interval<i64> = "2-4".parse().unwrap();
    let interval68: Interval<i64> = "6-8".parse().unwrap();
    assert_eq!(interval24.is_subset(&interval68), false);

    let interval24: Interval<i64> = "2-4".parse().unwrap();
    let interval28: Interval<i64> = "2-8".parse().unwrap();
    assert_eq!(interval24.is_subset(&interval28), true);
}
