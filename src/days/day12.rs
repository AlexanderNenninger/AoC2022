use std::{fmt::Display, process::id, str::FromStr};

use crate::{
    etc::{ErasedError, Matrix},
    Solution, SolutionPair,
};

///////////////////////////////////////////////////////////////////////////////
///
#[derive(Debug, Clone)]
struct PathProblem<const M: usize, const N: usize>
where
    [u8; M * N]:,
{
    map: Matrix<M, N, u8>,
    start_index: (usize, usize),
    end_index: (usize, usize),
}

impl<const M: usize, const N: usize> PathProblem<M, N>
where
    [u8; M * N]:,
{
    fn neighbors(&self, idx: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let [i, j] = idx;
        let mut row_indices = [None; 4];
        let mut col_indices = [None; 4];
        if i > 0 && i < M {
            row_indices[0] = Some(i - 1);
            col_indices[0] = Some(j);
        }
        if j > 0 && j < N {
            row_indices[1] = Some(i);
            col_indices[1] = Some(j - 1);
        }

        if j < N - 1 {
            row_indices[2] = Some(i);
            col_indices[2] = Some(j + 1);
        }

        if i < M - 1 {
            row_indices[3] = Some(i + 1);
            col_indices[3] = Some(j)
        }
        row_indices
            .into_iter()
            .zip(col_indices.into_iter())
            .filter_map(|(idx, idy)| Some([idx?, idy?]))
    }

    fn moves(&self, idx: [usize; 2]) -> impl Iterator<Item = [usize; 2]> + '_ {
        self.neighbors(idx).filter_map(move |n_idx| {
            if self.map[n_idx] - self.map[idx] <= 1 {
                return Some(idx);
            } else {
                None
            }
        })
    }
}

impl<const M: usize, const N: usize> FromStr for PathProblem<M, N>
where
    [u8; M * N]:,
{
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const START_CHAR: u8 = 'S' as u8;
        const END_CHAR: u8 = 'E' as u8;

        const MSG: &str = "ERROR: Parsing HeighMap failed.";
        let s = s.trim();
        if !s.trim().len() == M * N + N - 1 {
            return Err(MSG.into());
        }
        let mut map = Matrix::new(0);

        let mut start_index = None;
        let mut end_index = None;
        for (idx, mut height) in s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c as u8)
            .enumerate()
        {
            if height == START_CHAR {
                height = 'a' as u8;
                start_index = Some((idx / N, idx % N))
            } else if height == END_CHAR {
                height = 'z' as u8;
                end_index = Some((idx / N, idx % N))
            }
            map[idx] = height;
        }
        if let Some(start_index) = start_index {
            if let Some(end_index) = end_index {
                return Ok(PathProblem {
                    map: map,
                    start_index,
                    end_index,
                });
            }
        }
        Err(MSG.into())
    }
}

impl<const M: usize, const N: usize> Display for PathProblem<M, N>
where
    [(); M * N]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, h) in self.map.iter().enumerate() {
            if i % N == 0 {
                writeln!(f)?
            }
            if i / N == self.start_index.0 && i % N == self.start_index.1 {
                write!(f, "\x1b[36;1m {h:3}\x1b[0m")?
            } else if i / N == self.end_index.0 && i % N == self.end_index.1 {
                write!(f, "\x1b[35;1m {h:3}\x1b[0m")?
            } else {
                write!(f, " {h:3}")?
            }
        }
        writeln!(f)
    }
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod tests {

    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day12_test.txt");

    #[test]
    fn from_str() {
        let pp: PathProblem<5, 8> = TEST_INPUT.parse().unwrap();
        println!("{pp}");
    }

    #[test]
    fn neighbors() {
        const M: usize = 5;
        const N: usize = 8;
        let pp: PathProblem<M, N> = TEST_INPUT.parse().unwrap();

        let n_ul: Vec<[usize; 2]> = pp.neighbors([0, 0]).collect();
        assert_eq!(n_ul, vec![[0, 1], [1, 0]]);

        let n_lr_out_of_bounds: Vec<[usize; 2]> = pp.neighbors([5, 8]).collect();
        let v: Vec<[usize; 2]> = vec![];
        assert_eq!(n_lr_out_of_bounds, v);
    }
}
