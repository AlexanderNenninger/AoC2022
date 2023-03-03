use std::str::FromStr;

use crate::{
    etc::{ErasedError, Matrix},
    Solution, SolutionPair,
};

///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day14.txt");

#[derive(Debug, Clone)]
struct Cave<const M: usize, const N: usize>
where
    [(); N * M]:,
{
    active_sand_idx: [usize; 2],
    x_offset: usize,
    floor_level: usize,
    map: Box<Matrix<N, M, char>>,
}

impl<const M: usize, const N: usize> FromStr for Cave<M, N>
where
    [(); N * M]:,
{
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Box::new(Matrix::new('.'));

        let x_offset = 500 - N;

        let floor_level = s
            .lines()
            .map(|l| {
                l.split(" -> ")
                    .filter_map(|s| s.split_once(','))
                    .filter_map(|(i, j)| Some((i.parse::<usize>().ok()?, j.parse::<usize>().ok()?)))
                    .map(|(_, y)| y)
                    .max()
                    .unwrap_or(0)
            })
            .max()
            .unwrap_or(0)
            + 2; // Floor is places 2 below the bottom floor.

        if floor_level >= N {
            return Err(format!(
                "ERROR: Floor at level {} out of bounds ({}, {}).",
                floor_level, N, M
            )
            .into());
        }

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
        // Sand spawns at (500, 0);
        let active_sand_idx = [0, 500 - x_offset];
        Ok(Cave {
            active_sand_idx,
            x_offset,
            floor_level,
            map,
        })
    }
}

impl<const M: usize, const N: usize> Cave<M, N>
where
    [(); N * M]:,
{
    fn spawn_sand(&mut self) -> bool {
        self.active_sand_idx = [0, 500 - self.x_offset];
        if self.map[self.active_sand_idx] == '.' {
            self.map[self.active_sand_idx] = '+';
            return true;
        }
        false
    }

    fn step(&mut self) -> bool {
        // We have exited the map
        if self.look_down() == ' ' {
            return false;
        } else if self.look_down() == '.' {
            self.map[self.active_sand_idx] = '.';
            self.active_sand_idx[0] += 1;
            self.map[self.active_sand_idx] = '+';
            return true;
        } else if self.look_down_left() == '.' {
            self.map[self.active_sand_idx] = '.';
            self.active_sand_idx[0] += 1;
            self.active_sand_idx[1] -= 1;
            self.map[self.active_sand_idx] = '+';
            return true;
        } else if self.look_down_right() == '.' {
            self.map[self.active_sand_idx] = '.';
            self.active_sand_idx[0] += 1;
            self.active_sand_idx[1] += 1;
            self.map[self.active_sand_idx] = '+';
            return true;
        } else {
            self.map[self.active_sand_idx] = 'o';
            return self.spawn_sand();
        }
    }

    #[inline]
    fn look_down(&self) -> char {
        let [mut y, x] = self.active_sand_idx;
        y += 1;
        *self.map.get(y, x).unwrap_or(&' ')
    }

    #[inline]
    fn look_down_left(&self) -> char {
        let [mut y, mut x] = self.active_sand_idx;
        y += 1;
        x -= 1;
        *self.map.get(y, x).unwrap_or(&' ')
    }

    #[inline]
    fn look_down_right(&self) -> char {
        let [mut y, mut x] = self.active_sand_idx;
        y += 1;
        x += 1;
        *self.map.get(y, x).unwrap_or(&' ')
    }

    fn count_sand(&self) -> u64 {
        self.map.iter().map(|el| (*el == 'o') as u64).sum()
    }

    fn add_floor(&mut self) {
        for x in 0..M {
            self.map[[self.floor_level, x]] = '#';
        }
    }
}

fn part_1<const M: usize, const N: usize>(cave: &mut Cave<M, N>) -> u64
where
    [(); N * M]:,
{
    while cave.step() {}
    cave.count_sand()
}

fn part_2<const M: usize, const N: usize>(cave: &mut Cave<M, N>) -> u64
where
    [(); N * M]:,
{
    cave.add_floor();
    while cave.step() {}
    cave.count_sand()
}

pub fn solve() -> SolutionPair {
    let mut cave_1: Cave<320, 160> = INPUT.parse().unwrap();
    let mut cave_2 = cave_1.clone();
    let sol1: u64 = part_1(&mut cave_1);
    let sol2: u64 = part_2(&mut cave_2);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
    const TEST_M: usize = 24; // 2 * N - 1
    const TEST_N: usize = 12;

    #[test]
    fn from_str() {
        // We get the generics right. Kinda hadr to find good test conditions.
        let mut cave: Cave<TEST_M, TEST_N> = TEST_INPUT.parse().unwrap();
        assert_eq!(cave.floor_level, 11);
        cave.add_floor();
        println!("{}", cave.map);
    }

    #[test]
    fn from_str_large() {
        let cave: Cave<320, 160> = INPUT.parse().unwrap();
        println!("{}", cave.map);
    }

    #[test]
    fn test_part_1() {
        let mut cave: Cave<TEST_M, TEST_N> = TEST_INPUT.parse().unwrap();

        cave.spawn_sand();
        println!("{}", cave.map);

        while cave.step() {
            thread::sleep(Duration::from_millis(1000 / 30));
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", cave.map);
        }
        assert_eq!(cave.count_sand(), 24);
    }

    #[test]
    fn test_part_2() {
        let mut cave: Cave<TEST_M, TEST_N> = TEST_INPUT.parse().unwrap();
        cave.add_floor();
        cave.spawn_sand();
        println!("{}", cave.map);

        while cave.step() {
            thread::sleep(Duration::from_millis(1000 / 30));
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", cave.map);
        }
        assert_eq!(cave.count_sand(), 93);
    }
}
