use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////
struct AlphabetMultiset {
    data: u64,
}

impl AlphabetMultiset {
    fn new() -> Self {
        AlphabetMultiset { data: 0 }
    }

    fn index(c: u8) -> u8 {
        if c.is_ascii_lowercase() {
            return c as u8 - 'a' as u8;
        }
        panic!("ERROR: Passed non-ascii non-lowercase character.")
    }

    fn push(&mut self, c: u8) {
        self.data |= 1 << Self::index(c)
    }

    fn len(&self) -> u32 {
        self.data.count_ones()
    }
}

fn find_packet_start<const WINDOW: usize>(input: &str) -> u64 {
    for (idx, bs) in input.as_bytes().array_windows::<WINDOW>().enumerate() {
        let mut already_seen = AlphabetMultiset::new();
        for b in bs {
            already_seen.push(*b);
        }
        if already_seen.len() == WINDOW as u32 {
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
    assert_eq!(sol, 10)
}
