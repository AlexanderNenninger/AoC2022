use std::process::id;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
struct AlphabetMultiset {
    data: [u8; 26],
}

impl AlphabetMultiset {
    fn new() -> Self {
        AlphabetMultiset { data: [0; 26] }
    }

    fn index(c: char) -> usize {
        if c.is_ascii_lowercase() {
            return c as usize - 'a' as usize;
        }
        panic!("ERROR: Passed non-ascii non-lowercase character.")
    }

    fn push(&mut self, c: char) {
        self.data[Self::index(c)] += 1
    }

    fn all_unique(&self) -> bool {
        for b in self.data {
            if b > 1 {
                return false;
            }
        }
        true
    }
}

fn find_packet_start<const WINDOW: usize>(input: &str) -> u64 {
    let chars: Vec<char> = input.chars().collect();
    for (idx, cs) in chars.array_windows::<WINDOW>().enumerate() {
        let mut already_seen = AlphabetMultiset::new();

        for c in cs {
            already_seen.push(*c);
        }
        if already_seen.all_unique() {
            return idx as u64 + WINDOW as u64;
        }
    }
    0
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    const INPUT: &str = include_str!("../../input/day06.txt");
    let sol1: u64 = find_packet_start::<4>(INPUT);
    let sol2: u64 = find_packet_start::<14>(INPUT);

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[test]
fn test_part_1() {
    let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let sol = find_packet_start::<4>(input);
    assert_eq!(sol, 5);
    let input = "nppdvjthqldpwncqszvftbrmjlhg";
    let sol = find_packet_start::<4>(input);
    assert_eq!(sol, 6);
    let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let sol = find_packet_start::<4>(input);
    assert_eq!(sol, 10);
}
