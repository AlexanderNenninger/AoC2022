use std::{fmt::Display, str::FromStr};

use crate::{etc::ErasedError, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq)]
struct Woods<const N: usize> {
    trees: [[i8; N]; N],
}

impl<const N: usize> Woods<N> {
    fn visible_left(&self) -> [[bool; N]; N] {
        let mut visible = [[true; N]; N];
        for i in 0..N {
            let mut max_height_so_far = -1;
            for j in 0..N {
                let tree = self.trees[i][j];
                if tree <= max_height_so_far {
                    visible[i][j] = false
                }
                max_height_so_far = max_height_so_far.max(tree)
            }
        }
        visible
    }
    fn visible_right(&self) -> [[bool; N]; N] {
        let mut visible = [[true; N]; N];
        for i in 0..N {
            let mut max_height_so_far = -1;
            for j in 0..N {
                let tree = self.trees[i][N - j - 1];
                if tree <= max_height_so_far {
                    visible[i][N - j - 1] = false
                }
                max_height_so_far = max_height_so_far.max(tree)
            }
        }
        visible
    }
    fn visible_top(&self) -> [[bool; N]; N] {
        let mut visible = [[true; N]; N];
        for i in 0..N {
            let mut max_height_so_far = -1;
            for j in 0..N {
                let tree = self.trees[j][i];
                if tree <= max_height_so_far {
                    visible[j][i] = false
                }
                max_height_so_far = max_height_so_far.max(tree)
            }
        }
        visible
    }
    fn visible_bottom(&self) -> [[bool; N]; N] {
        let mut visible = [[true; N]; N];
        for i in 0..N {
            let mut max_height_so_far = -1;
            for j in 0..N {
                let tree = self.trees[N - j - 1][i];
                if tree <= max_height_so_far {
                    visible[N - j - 1][i] = false
                }
                max_height_so_far = max_height_so_far.max(tree)
            }
        }
        visible
    }

    fn visible(&self) -> [[bool; N]; N] {
        let mut visible = [[false; N]; N];
        let vl = self.visible_left();
        let vr = self.visible_right();
        let vt = self.visible_top();
        let vb = self.visible_bottom();
        for i in 0..N {
            for j in 0..N {
                visible[i][j] = vl[i][j] || vr[i][j] || vt[i][j] || vb[i][j]
            }
        }
        visible
    }

    fn scenic_score(&self, i: usize, j: usize) -> u64 {
        let tree_height = self.trees[i][j];
        // looking north
        let mut k = 1;
        let mut visible_up = 0;
        while i as isize - k as isize >= 0 {
            k += 1;
            if self.trees[i + 1 - k][j] >= tree_height {
                visible_up += 1;
                break;
            }
            visible_up += 1;
        }
        // looking west
        let mut k = 1;
        let mut visible_left = 0;
        while j as isize - k as isize >= 0 {
            k += 1;
            if self.trees[i][j + 1 - k] >= tree_height {
                visible_left += 1;
                break;
            }
            visible_left += 1;
        }
        // looking east
        let mut k = 1;
        let mut visible_right = 0;
        while j + k < N {
            k += 1;
            if self.trees[i][j + k - 1] >= tree_height {
                visible_right += 1;
                break;
            }
            visible_right += 1;
        }
        // looking down
        let mut k = 1;
        let mut visible_down = 0;
        while i + k < N {
            k += 1;
            if self.trees[i + k - 1][j] >= tree_height {
                visible_down += 1;
                break;
            }
            visible_down += 1;
        }
        visible_up * visible_left * visible_right * visible_down
    }
}

impl<const N: usize> FromStr for Woods<N> {
    type Err = ErasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = [[0i8; N]; N];
        for (i, line) in s.trim().lines().enumerate() {
            for (j, val) in line.as_bytes().iter().enumerate() {
                trees[i][j] = (val - '0' as u8) as i8
            }
        }
        Ok(Woods { trees })
    }
}

impl<const N: usize> Display for Woods<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const ESCAPE: char = 27 as char;
        let vs = self.visible();
        let mut out: String = "".into();
        for i in 0..N {
            for j in 0..N {
                if vs[i][j] {
                    out.push_str(&format!("{ESCAPE}[42m"));
                } else {
                    out.push_str(&format!("{ESCAPE}[44m"));
                }
                out.push((self.trees[i][j] as u8 + '0' as u8) as char);
                out.push_str(&format!("{ESCAPE}[0m"));
            }
            if i < N - 1 {
                out.push('\n')
            }
        }
        writeln!(f, "{}", out)
    }
}

fn part_1<const N: usize>(input: &str) -> u64 {
    let woods: Woods<N> = input.parse().unwrap();
    let visible = woods.visible();

    let mut acc = 0;
    for i in 0..N {
        for j in 0..N {
            acc += visible[i][j] as u64
        }
    }

    acc
}

fn part_2<const N: usize>(input: &str) -> u64 {
    let woods: Woods<N> = input.parse().unwrap();
    let mut sc = 0;
    for i in 1..N - 1 {
        for j in 1..N - 1 {
            sc = sc.max(woods.scenic_score(i, j))
        }
    }
    sc
}

pub fn solve() -> SolutionPair {
    const INPUT: &str = include_str!("../../input/day08.txt");
    let sol1: u64 = part_1::<99>(INPUT);
    let sol2: u64 = part_2::<99>(INPUT);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_part_1() {
    let input = include_str!("../../input/day08_test.txt");
    let woods: Woods<5> = input.parse().unwrap();
    assert_eq!(woods.trees[0], [3, 0, 3, 7, 3,]);

    let count_visible = part_1::<5>(input);
    assert_eq!(count_visible, 21);
}

#[test]
fn test_part_2() {
    let input = include_str!("../../input/day08_test.txt");
    let woods: Woods<5> = input.parse().unwrap();
    assert_eq!(woods.trees[0], [3, 0, 3, 7, 3,]);

    let scenic_score = woods.scenic_score(1, 2);
    assert_eq!(scenic_score, 4);
    let scenic_score = woods.scenic_score(3, 2);
    assert_eq!(scenic_score, 8);
}
