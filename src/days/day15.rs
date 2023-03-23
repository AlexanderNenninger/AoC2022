use std::str::FromStr;

use crate::{
    etc::{ErasedError, Point},
    Solution, SolutionPair,
};

///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day15.txt");

type Pi64 = Point<i64>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ball {
    center: Pi64,
    radius: i64,
}

impl Ball {
    fn contains(&self, point: &Pi64) -> bool {
        self.center.dist_1(point) <= self.radius
    }

    /// Map t to point on perimeter of the ball counter clock-wise.
    fn perimeter(&self, t: i64) -> Pi64 {
        let r_perimeter = self.radius + 1;

        let quadrant = t / (r_perimeter);
        let step = t % (r_perimeter);

        match quadrant % 4 {
            0 => self.center + Pi64::new(r_perimeter, 0) + Pi64::new(-1, -1) * step,
            1 => self.center + Pi64::new(0, -r_perimeter) + Pi64::new(-1, 1) * step,
            2 => self.center + Pi64::new(-r_perimeter, 0) + Pi64::new(1, 1) * step,
            3 => self.center + Pi64::new(0, r_perimeter) + Pi64::new(1, -1) * step,
            _ => unreachable!(),
        }
    }

    fn iter_perimeter(&self) -> IterPerimeter {
        IterPerimeter { ball: &self, t: 0 }
    }
}

struct IterPerimeter<'a> {
    ball: &'a Ball,
    t: i64,
}

impl<'a> Iterator for IterPerimeter<'a> {
    type Item = Pi64;

    fn next(&mut self) -> Option<Self::Item> {
        self.t += 1;
        // Stop iterating if we went around once
        if self.t <= 4 * (self.ball.radius + 1) {
            Some(self.ball.perimeter(self.t - 1))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pair {
    sensor: Ball,
    beacon: Pi64,
}

impl FromStr for Pair {
    type Err = ErasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum ParserState {
            ReadingNumber,
            Scanning,
        }
        use ParserState::*;
        let mut parser_state = Scanning;

        let mut number_stack = String::new();
        let mut numbers: Vec<i64> = Vec::with_capacity(4);
        for c in s.chars() {
            match parser_state {
                Scanning => match c {
                    '=' => parser_state = ReadingNumber,
                    _ => {}
                },
                ReadingNumber => match c {
                    c if c.is_digit(10) => number_stack.push(c),
                    '-' => number_stack.push('-'),
                    _ => {
                        numbers.push(number_stack.parse()?);
                        number_stack.clear();
                        parser_state = Scanning
                    }
                },
            }
        }
        if !number_stack.is_empty() {
            numbers.push(number_stack.parse()?);
            number_stack.clear()
        }
        let err_msg = "ERROR: Not enough coordinates found.";
        let bx = numbers.get(0).ok_or(err_msg.to_string())?;
        let by = numbers.get(1).ok_or(err_msg.to_string())?;
        let sx = numbers.get(2).ok_or(err_msg.to_string())?;
        let sy = numbers.get(3).ok_or(err_msg.to_string())?;

        let center = Pi64::new(*bx, *by);
        let beacon = Pi64::new(*sx, *sy);
        let radius = center.dist_1(&beacon);
        let sensor = Ball { center, radius };

        Ok(Pair { sensor, beacon })
    }
}

fn part_1(pairs: &Vec<Pair>, y: i64) -> Option<u64> {
    let x_min = pairs
        .iter()
        .map(|b| b.sensor.center.x - b.sensor.radius)
        .min()?;
    let x_max = pairs
        .iter()
        .map(|b| b.sensor.center.x + b.sensor.radius)
        .max()?;

    let mut p = Pi64::new(x_min, y);
    let mut cntr = 0;
    for x in x_min..=x_max {
        p.x = x;
        let mut no_beacon = false;
        for Pair { sensor, beacon } in pairs {
            if *beacon == p {
                no_beacon = false;
                break;
            }
            if no_beacon || sensor.contains(&p) {
                no_beacon = true;
            }
        }
        cntr += no_beacon as u64
    }
    Some(cntr)
}

fn part_2(pairs: &Vec<Pair>, x_max: i64, y_max: i64) -> Option<u64> {
    let sensors = pairs.iter().map(|p| &p.sensor);

    for sensor in sensors.clone() {
        for point in sensor.iter_perimeter() {
            if point.x < 0 || point.x > x_max || point.y < 0 || point.y > y_max {
                continue;
            }

            let mut no_sensor = true;
            for sensor in sensors.clone() {
                if sensor.contains(&point) {
                    no_sensor = false;
                    break;
                }
            }

            if no_sensor {
                return Some(point.x as u64 * 4000000 + point.y as u64);
            }
        }
    }

    None
}

pub fn solve() -> SolutionPair {
    // Your solution here...
    let pairs: Vec<Pair> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
    let sol1: u64 = part_1(&pairs, 2000000).unwrap();
    let sol2: u64 = part_2(&pairs, 4000000, 4000000).unwrap();

    (Solution::U64(sol1), Solution::U64(sol2))
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day15_test.txt");

    #[test]
    fn parse_ball() {
        let data = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let p: Pair = data.parse().unwrap();
        assert_eq!(p.sensor.center, Pi64::new(2, 18));
        assert_eq!(p.sensor.radius, 7);
    }

    #[test]
    fn test_part_1() {
        let pairs: Vec<Pair> = TEST_INPUT.lines().map(|l| l.parse().unwrap()).collect();
        let sol = part_1(&pairs, 10).unwrap();
        assert_eq!(sol, 26);
    }

    #[test]
    fn test_part_2() {
        let pairs: Vec<Pair> = TEST_INPUT.lines().map(|l| l.parse().unwrap()).collect();
        let sol = part_2(&pairs, 20, 20).unwrap();
        assert_eq!(sol, 56000011);
    }

    #[test]
    fn test_iter_perimeter() {
        let r = 1;

        let b = Ball {
            center: Pi64::new(0, 0),
            radius: r,
        };

        let points: Vec<_> = b.iter_perimeter().collect();
        assert_eq!(points.len() as i64, 4 * (r + 1))
    }
}
