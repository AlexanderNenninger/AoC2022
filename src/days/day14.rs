use std::str::FromStr;

use crate::{
    etc::{ErasedError, Matrix},
    Solution, SolutionPair,
};

///////////////////////////////////////////////////////////////////////////////

struct Cave<const M: usize, const N: usize>
where
    [(); N * M]:,
{
    x_offset: usize,
    map: Box<Matrix<N, M, char>>,
}

impl<const M: usize, const N: usize> FromStr for Cave<M, N>
where
    [(); N * M]:,
{
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Box::new(Matrix::new('.'));

        let x_offset = s
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .filter_map(|s| s.split_once(','))
                    .filter_map(|(i, j)| Some((i.parse::<usize>().ok()?, j.parse::<usize>().ok()?)))
                    .map(|(x, y)| x)
                    .min()
                    .unwrap_or(0)
            })
            .min()
            .unwrap_or(0)
            .saturating_sub(5);

        for line in s.lines() {
            let steps: Vec<(usize, usize)> = line
                .split(" -> ")
                .filter_map(|s| s.split_once(','))
                .filter_map(|(i, j)| Some((i.parse::<usize>().ok()?, j.parse::<usize>().ok()?)))
                .collect();

            if steps.len() < 2 {
                return Err("ERROR: Path too short.".into());
            }

            for [prev, snd] in steps.array_windows::<2>() {
                for x in prev.0.min(snd.0)..=prev.0.max(snd.0) {
                    for y in prev.1.min(snd.1)..=prev.1.max(snd.1) {
                        map[[y, x - x_offset]] = '#'
                    }
                }
            }
        }
        Ok(Cave { x_offset, map })
    }
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {
        let data = "4,4 -> 4,6 -> 2,6\n9,4 -> 8,4 -> 8,9 -> 0,9";
        let cave: Cave<10, 10> = data.parse().unwrap();
        println!("{}", cave.map);
    }

    #[test]
    fn from_str_large() {
        let data = include_str!("../../input/day14.txt");
        let cave: Cave<80, 160> = data.parse().unwrap();
        println!("{}", cave.map);
    }
}
