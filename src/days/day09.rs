use std::{collections::HashSet, str::FromStr};

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

impl Move {
    fn get_inner(&self) -> isize {
        match self {
            Move::Up(n) => *n,
            Move::Down(n) => *n,
            Move::Left(n) => *n,
            Move::Right(n) => *n,
        }
    }
}

impl FromStr for Move {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, n_steps) = s
            .split_once(" ")
            .ok_or("ERROR: Could not split move input.".to_string())?;
        let n_steps: isize = n_steps.parse()?;
        Ok(match dir {
            "U" => Self::Up(n_steps),
            "D" => Self::Down(n_steps),
            "L" => Self::Left(n_steps),
            "R" => Self::Right(n_steps),
            _ => Err("ERROR: Unknown direction identifier.".to_string())?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rope<const N: usize> {
    knots: [(i64, i64); N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope { knots: [(0, 0); N] }
    }

    fn step_one(&mut self, m: Move) {
        match m {
            Move::Up(_) => self.knots[0].0 += 1,
            Move::Down(_) => self.knots[0].0 -= 1,
            Move::Left(_) => self.knots[0].1 -= 1,
            Move::Right(_) => self.knots[0].1 += 1,
        }
        self.drag_tail();
    }

    fn step(&mut self, m: Move, visited: &mut HashSet<(i64, i64)>) {
        for _ in 0..m.get_inner() {
            self.step_one(m);
            visited.insert(self.knots[N - 1]);
        }
    }

    fn drag_tail(&mut self) {
        for i in 1..N {
            let vdiff = self.knots[i - 1].0 - self.knots[i].0;
            let hdiff = self.knots[i - 1].1 - self.knots[i].1;

            if vdiff.abs() > 1 && hdiff.abs() > 0 {
                self.knots[i].0 += vdiff.signum();
                self.knots[i].1 += hdiff.signum();
            } else if vdiff.abs() > 0 && hdiff.abs() > 1 {
                self.knots[i].0 += vdiff.signum();
                self.knots[i].1 += hdiff.signum();
            } else if vdiff.abs() > 1 {
                self.knots[i].0 += vdiff.signum()
            } else if hdiff.abs() > 1 {
                self.knots[i].1 += hdiff.signum()
            }
        }
    }
}

fn _show_rope<const M: usize, const N: usize, const K: usize>(rope: &Rope<K>) {
    for i in (0..M).rev() {
        for j in 0..N {
            if (i as i64, j as i64) == rope.knots[0] {
                print!("H")
            } else if (i as i64, j as i64) == rope.knots[K] {
                print!("T")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}

fn _show_visited<const M: usize, const N: usize>(visited: &HashSet<(i64, i64)>) {
    for i in (0..M).rev() {
        for j in 0..N {
            if visited.contains(&(i as i64, j as i64)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}
fn simulate_rope<const K: usize>(input: &str) -> u64 {
    let mut visited = HashSet::new();
    let moves: Vec<Move> = input
        .trim()
        .lines()
        .map(|l| l.parse().expect("ERROR: Could not parse move."))
        .collect();
    let mut rope = Rope::<K>::new();
    visited.insert(rope.knots[K - 1]);

    for m in moves {
        rope.step(m, &mut visited)
    }

    visited.len() as u64
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day09.txt");
    let sol1: u64 = simulate_rope::<2>(INPUT);
    let sol2: u64 = simulate_rope::<10>(INPUT);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_part_1() {
    const INPUT: &str = include_str!("../../input/day09_test.txt");
    let num_visited = simulate_rope::<2>(INPUT);
    assert_eq!(num_visited, 13);
}
#[test]
fn test_part_2() {
    const INPUT: &str = include_str!("../../input/day09_test.txt");
    let num_visited = simulate_rope::<10>(INPUT);
    assert_eq!(num_visited, 1);
}
