#![allow(unused)]
use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
    ops::{Index, IndexMut},
};
use Solution::*;

pub type ErasedError = Box<dyn Error + Send + Sync + 'static>;

pub enum Solution {
    I32(i32),
    I64(i64),
    I128(i128),
    U32(u32),
    U64(u64),
    U128(u128),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            I32(x) => x.fmt(f),
            I64(x) => x.fmt(f),
            I128(x) => x.fmt(f),
            U32(x) => x.fmt(f),
            U64(x) => x.fmt(f),
            U128(x) => x.fmt(f),
            Str(x) => x.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Matrix<const M: usize, const N: usize, T>([T; M * N])
where
    [T; M * N]:;

impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    [T; M * N]:,
{
    fn _index(i: usize, j: usize) -> usize {
        i * N + j
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < M && j < N {
            return Some(&self.0[Self::_index(i, j)]);
        }
        None
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        if i < M && j < N {
            return Some(&mut self.0[Self::_index(i, j)]);
        }
        None
    }

    pub fn each_index(&self) -> impl Iterator<Item = [usize; 2]> {
        (0..M).flat_map(|i| (0..N).map(move |j| [i, j]))
    }
}

impl<const M: usize, const N: usize, T: Copy> Matrix<M, N, T>
where
    [T; M * N]:,
{
    pub fn new(val: T) -> Self {
        Matrix([val; M * N])
    }
}

impl<const M: usize, const N: usize, T> std::ops::Deref for Matrix<M, N, T>
where
    [T; M * N]:,
{
    type Target = [T; M * N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const M: usize, const N: usize, T> std::ops::DerefMut for Matrix<M, N, T>
where
    [T; M * N]:,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const M: usize, const N: usize, T> Index<[usize; 2]> for Matrix<M, N, T>
where
    [(); M * N]:,
{
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [i, j] = index;
        let idx = Self::_index(i, j);
        &self.0[idx]
    }
}

impl<const M: usize, const N: usize, T> Index<usize> for Matrix<M, N, T>
where
    [(); M * N]:,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const M: usize, const N: usize, T> IndexMut<usize> for Matrix<M, N, T>
where
    [(); M * N]:,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn each_index() {
        const M: usize = 5;
        const N: usize = 7;
        let mat: Matrix<M, N, u8> = Matrix::new(0);
        let idxs: Vec<_> = mat.each_index().collect();
        assert_eq!(idxs.len(), M * N)
    }
}
