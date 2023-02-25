use std::str::FromStr;

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    PLUS(i64),
    MUL(i64),
    SQUARE,
}

impl Operation {
    fn apply(&self, i: i64) -> i64 {
        match self {
            Self::PLUS(a) => i + a,
            Self::MUL(a) => i * a,
            Self::SQUARE => i * i,
        }
    }
}

impl FromStr for Operation {
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MSG: &str = "Could not parse operation";
        let symb_idx = s.find(|c| c == '+' || c == '*').ok_or(ERROR_MSG)?;
        let symbol = s.chars().nth(symb_idx).ok_or(ERROR_MSG)?;

        let mut s_chars = s.chars();
        s_chars.advance_by(symb_idx + 1).or(Err(ERROR_MSG))?;
        let rest = s_chars.collect::<String>();

        if rest.trim() == "old" {
            return Ok(Self::SQUARE);
        }

        let c: i64 = rest.trim().parse()?;

        match symbol {
            '+' => Ok(Self::PLUS(c)),
            '*' => Ok(Self::MUL(c)),
            _ => Err(ERROR_MSG.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    id: usize,
    inspection_count: usize,
    items: Vec<i64>,
    operation: Operation,
    test_condition: i64,
    target_monkeys: (usize, usize), // Index into array of Monkeys
}

impl FromStr for Monkey {
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MSG: &str = "ERROR: Parsing monkey failed";
        let mut lines = s.lines();

        let header_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(header_str);

        let id: usize = header_str
            .split_once(' ')
            .ok_or(ERROR_MSG)?
            .1
            .trim_end_matches(":")
            .parse()?;

        let item_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(&item_str);

        let items = item_str
            .chars()
            .filter(|c| c.is_numeric() || c.is_whitespace())
            .collect::<String>()
            .trim()
            .split(" ")
            .map(|s| s.trim().parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()?;

        let op_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(op_str);
        let operation: Operation = op_str.parse()?;

        let test_condition_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(test_condition_str);

        let (_, test_condition_str) = test_condition_str.rsplit_once(" ").ok_or(ERROR_MSG)?;
        let test_condition: i64 = test_condition_str.parse()?;

        let target_monkey_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(target_monkey_str);

        let (_, target_monkey_str) = target_monkey_str.rsplit_once(" ").ok_or(ERROR_MSG)?;
        let target_monkey_1: usize = target_monkey_str.parse()?;

        let target_monkey_str = lines.next().ok_or(ERROR_MSG)?;
        dbg!(target_monkey_str);

        let (_, target_monkey_str) = target_monkey_str.rsplit_once(" ").ok_or(ERROR_MSG)?;
        let target_monkey_2: usize = target_monkey_str.parse()?;
        let target_monkeys = (target_monkey_1, target_monkey_2);

        let num_inspections = 0;

        Ok(Monkey {
            id,
            inspection_count: num_inspections,
            items,
            operation,
            test_condition,
            target_monkeys,
        })
    }
}

fn round(monkeys: &mut Vec<Monkey>, mod_by: i64, divide_by: i64) -> Result<(), ErasedError> {
    for i in 0..monkeys.len() {
        let monkey = &monkeys[i];
        let target_monkeys = monkey.target_monkeys;

        // get_many_mut ensures we're not aliasing any monkeys
        let [monkey, monkey_success, monkey_failure] =
            monkeys.get_many_mut::<3>([i, target_monkeys.0, target_monkeys.1])?;

        for j in 0..monkey.items.len() {
            let mut item = monkey.items[j];
            item = monkey.operation.apply(item);
            item = item / divide_by;
            item = item % mod_by;
            if item % monkey.test_condition == 0 {
                monkey_success.items.push(item)
            } else {
                monkey_failure.items.push(item)
            }
        }
        monkey.inspection_count += monkey.items.len();
        monkey.items.clear();
    }
    Ok(())
}

fn get_mod_val(monkeys: &Vec<Monkey>) -> i64 {
    monkeys.iter().map(|m| m.test_condition).product()
}

fn monkey_business(monkeys: &mut Vec<Monkey>, rounds: usize, divide_by: i64) -> u64 {
    let mod_val = get_mod_val(monkeys);

    for _ in 0..rounds {
        round(monkeys, mod_val, divide_by).unwrap();
    }
    let mut inspection_counts: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.inspection_count)
        .collect();

    inspection_counts.sort();
    (inspection_counts[inspection_counts.len() - 1]
        * inspection_counts[inspection_counts.len() - 2]) as u64
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let data = include_str!("../../input/day11.txt");
    let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();

    let sol1: u64 = monkey_business(&mut monkeys.clone(), 20, 3);
    let sol2: u64 = monkey_business(&mut monkeys, 10000, 1);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod tests {

    #[allow(unused)]
    use super::*;

    #[test]
    fn test_parse_operation() {
        let data = "Operation: new = old * 19";
        let operation: Operation = data.parse().unwrap();
        assert_eq!(operation, Operation::MUL(19));

        let data = "Operation: new = old * old";
        let operation: Operation = data.parse().unwrap();
        assert_eq!(operation, Operation::SQUARE);

        let data = "Operation: new = old + 333";
        let operation: Operation = data.parse().unwrap();
        assert_eq!(operation, Operation::PLUS(333));
    }

    #[test]
    fn test_parse_monkey() {
        let data: &str = "Monkey 0:\nStarting items: 79, 98\nOperation: new = old * 19\nTest: divisible by 23\nIf true: throw to monkey 2\nIf false: throw to monkey 3";
        let res = data.parse::<Monkey>();
        assert!(res.is_ok());
        dbg!(&res.unwrap());
    }

    #[test]
    fn test_monkey_rounds() {
        let data = include_str!("../../input/day11_test.txt");
        let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();

        round(&mut monkeys, 10000, 3).unwrap();

        for (idx, item) in [2080, 25, 167, 207, 401, 1046].iter().enumerate() {
            assert!(monkeys[1].items[idx] == *item);
        }
    }

    #[test]
    fn test_part_1() {
        let data = include_str!("../../input/day11_test.txt");
        let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();
        let res = monkey_business(&mut monkeys, 20, 3);
        assert_eq!(res, 10605);
    }

    #[test]
    fn test_part_2() {
        let data = include_str!("../../input/day11_test.txt");
        let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|s| s.parse().unwrap()).collect();
        let res = monkey_business(&mut monkeys, 10000, 1);
        assert_eq!(res, 2713310158);
    }
}
