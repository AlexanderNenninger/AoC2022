#![allow(unused)]
use std::{
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
    process::Output,
};

use num_traits::Signed;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Signed,
{
    pub fn norm_1(&self) -> T {
        self.x.abs() + self.y.abs()
    }
}

impl<T> Point<T>
where
    T: Sub<Output = T> + Add<Output = T> + Signed + Copy,
{
    pub fn dist_1(&self, other: &Self) -> T {
        (*self - *other).norm_1()
    }
}

impl<T: SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<'a, T> Sub for &'a Point<T>
where
    &'a T: Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, rhs: &'a Point<T>) -> Self::Output {
        Point {
            x: &self.x - &rhs.x,
            y: &self.y - &rhs.y,
        }
    }
}
impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<'a, T> Add for &'a Point<T>
where
    &'a T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: &self.x + &rhs.x,
            y: &self.y + &rhs.y,
        }
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_i64() {
        type P64 = Point<i64>;
        let p = P64::new(5, 7);
        let q = P64::new(12, 13);
        assert_eq!(p - q, P64::new(-7, -6));

        assert_eq!(p.norm_1(), 5 + 7);

        assert_eq!(p.dist_1(&q), 7 + 6);
        println!("{p:?}");
    }
}
