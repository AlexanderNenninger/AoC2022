use std::char::MAX;

use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

fn index_2d_mut<T>(i: usize, j: usize, ncols: usize, buf: &mut [T]) -> &mut T {
    &mut buf[i * ncols + j]
}
fn index_2d<T>(i: usize, j: usize, ncols: usize, buf: &[T]) -> &T {
    &buf[i * ncols + j]
}

const rock_cycle: &[Rock] = &[Rock {
    width: 4,
    height: 1,
    sprite: &[1, 1, 1, 1],
    position_x: usize::MAX,
    position_y: usize::MAX,
}];

struct Game<const WIDTH: usize> {
    rocks: Vec<Rock>,
}

#[derive(Debug, Clone, PartialEq)]
struct Rock {
    width: usize,
    height: usize,
    sprite: &'static [u8],
    position_x: usize,
    position_y: usize,
}

impl Rock {
    const MAXSIZE: usize = 4;

    const fn new(width: usize, sprite: &'static [u8], position_y: usize) -> Self {
        Rock {
            width,
            height: sprite.len() / width,
            sprite,
            position_x: 2,
            position_y,
        }
    }

    fn collide(&self, other: &Self) -> bool {
        // If there cannot be any overlap, skip check.
        if self.position_x.abs_diff(other.position_x) > self.width.max(other.width) {
            return false;
        }
        if self.position_y.abs_diff(other.position_y) > self.height.max(other.height) {
            return false;
        }

        let min_x = self.position_x.min(other.position_x);
        // Place self.sprite in field.
        let mut field = [0u8; 4 * Self::MAXSIZE * Self::MAXSIZE];
        for i in 0..self.height {
            for j in 0..self.width {
                let source = index_2d(i, j, self.width, self.sprite);
                let target = index_2d_mut(
                    i + self.position_y.abs_diff(other.position_y),
                    j + self.position_x.abs_diff(other.position_x),
                    2 * Self::MAXSIZE,
                    &mut field,
                );
                *target += *source;
            }
        }
        // Place other.sprite in field.
        let mut field = [0u8; 4 * Self::MAXSIZE * Self::MAXSIZE];
        for i in 0..other.height {
            for j in 0..other.width {
                let source = index_2d(i, j, other.width, other.sprite);
                let target = index_2d_mut(
                    i + other.position_y.abs_diff(self.position_y),
                    j + other.position_x.abs_diff(self.position_x),
                    2 * Self::MAXSIZE,
                    &mut field,
                );
                *target += *source;
            }
        }
        false
    }
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::U64(sol1), Solution::U64(sol2))
}

fn add2d(
    sprite: &[u8],
    pos: (usize, usize),
    size: (usize, usize),
    buf: &mut [u8],
    buf_size: (usize, usize),
) {
    let nrows = size.0;
    let ncols = size.1;
    let row_offset = pos.0;
    let col_offset = pos.1;

    let buf_cols = buf_size.0;
    for i in 0..nrows {
        for j in 0..ncols {
            let source = index_2d(i, j, ncols, sprite);
            let target = index_2d_mut(i + row_offset, j + col_offset, buf_cols, buf);
            *target += *source;
        }
    }
}

#[test]
fn test_index2d() {
    let buf: [u64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let six = index_2d(1, 2, 3, &buf);
    assert_eq!(six, &6);
}

#[test]
fn test_add2d() {
    // ____
    // 010_
    // 111_
    let pos_x = 0;
    let pos_y = 1;
    let mut buf = [0u8; 4 * 4];
    let sprite = [0, 1, 0, 1, 1, 1];
    add2d(&sprite, (pos_y, pos_x), (2, 3), &mut buf, (4, 4));
    assert_eq!(*index_2d(1, 0, 4, &buf), 0);
    assert_eq!(*index_2d(1, 1, 4, &buf), 1);
    assert_eq!(*index_2d(1, 2, 4, &buf), 0);
    assert_eq!(*index_2d(2, 0, 4, &buf), 1);
    assert_eq!(*index_2d(2, 1, 4, &buf), 1);
    assert_eq!(*index_2d(2, 2, 4, &buf), 1);
}

fn dynamic_sprite_collision<const MAX_SIZE: usize>(
    s: &[u8],
    s_size: (usize, usize),
    s_pos: (usize, usize),
    t: &[u8],
    t_size: (usize, usize),
    t_pos: (usize, usize),
) -> bool
where
    [(); MAX_SIZE * MAX_SIZE]:,
{
    let (min_row_pos, min_col_pos) = (s_pos.0.min(t_pos.0), s_pos.1.min(t_pos.1));
    let s_pos = (s_pos.0 - min_row_pos, s_pos.1 - min_col_pos);
    let t_pos = (t_pos.0 - min_row_pos, t_pos.1 - min_col_pos);

    let mut buf = [0u8; MAX_SIZE * MAX_SIZE];
    add2d(s, s_pos, s_size, &mut buf, (MAX_SIZE, MAX_SIZE));
    add2d(t, t_pos, t_size, &mut buf, (MAX_SIZE, MAX_SIZE));
    buf.iter().any(|&x| x > 1)
}

#[test]
fn test_dynamic_sprite_collision() {
    // 010
    // 111
    const MAX_SIZE: usize = 2 * 3;
    let s = [0, 1, 0, 1, 1, 1];
    let s_pos = (10, 11);
    let s_size = (2, 3);

    // 10
    // 01
    let t = [1, 0, 0, 1];
    let t_pos = (11, 12);
    let t_size = (2, 2);

    // Goal:
    // 010000
    // 121000
    // 001000
    // 000000
    // 000000
    // 000000

    let res = dynamic_sprite_collision::<6>(&s, s_size, s_pos, &t, t_size, t_pos);
    assert!(res);
}
