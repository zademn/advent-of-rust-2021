use std::{collections::HashMap, ops::Add};

use crate::utils::read_challenge_data;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res,
    sequence::separated_pair, IResult,
};

/// Parser for the input
/// I could've used regex but I wanted to take a look at `nom`
#[allow(clippy::type_complexity)]
fn parser(input: &str) -> IResult<&str, ((isize, isize), (isize, isize))> {
    // Function that parse u32
    fn parse_isize(input: &str) -> Result<isize, std::num::ParseIntError> {
        input.parse()
    }
    // A pair splitted by " -> " of 2 pairs splitted by ',' of 2 digits
    // The digits are strings so I transform them to `u32` with `parse_u32`
    separated_pair(
        separated_pair(
            map_res(digit1, parse_isize),
            tag(","),
            map_res(digit1, parse_isize),
        ),
        tag(" -> "),
        separated_pair(
            map_res(digit1, parse_isize),
            tag(","),
            map_res(digit1, parse_isize),
        ),
    )(input)
}

/// Get a point that represents a `step` in the direction between 2 points.
fn get_direction(p: &Point, q: &Point) -> Point {
    let x = match p.x.cmp(&q.x) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Less => 1,
    };
    let y = match p.y.cmp(&q.y) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Less => 1,
    };
    Point { x, y }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    p: Point,
    q: Point,
    direction: Point,
}
impl Line {
    /// Parse the arrow notation and create a line from it
    fn from_arrow(s: &str) -> Self {
        let (_, ((x0, y0), (x1, y1))) = parser(s).unwrap();
        let p = Point { x: x0, y: y0 };
        let q = Point { x: x1, y: y1 };
        let direction = get_direction(&p, &q);
        Self { p, q, direction }
    }
    /// Returns all points from the line
    fn all_points(&self) -> Vec<Point> {
        let mut t = self.p;
        let mut points = Vec::new();
        while t != self.q {
            points.push(t);
            t = t + self.direction;
        }
        points.push(self.q);
        points
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(5, run_example);
    let mut lines: Vec<Line> = Vec::new();

    //println!("{:?}", parser("1,2 -> 1,10"));
    for line in input.lines() {
        lines.push(Line::from_arrow(line));
    }

    let mut point_freq_dict: HashMap<Point, u32> = HashMap::new();

    let horizontal_vertical = [
        Point { x: 0, y: 1 },
        Point { x: 0, y: -1 },
        Point { x: 1, y: 0 },
        Point { x: -1, y: 0 },
    ];

    for line in lines {
        // If we are doing part 1 skip the points that are on diagonals
        if part1 && !horizontal_vertical.contains(&line.direction) {
            continue;
        }
        for point in line.all_points() {
            let v = point_freq_dict.get(&point).unwrap_or(&0) + 1;
            point_freq_dict.insert(point, v);
        }
    }
    //println!("{:?}", point_freq_dict);
    let res = point_freq_dict.into_values().filter(|&e| e > 1).count();
    println!("{}", res); // 8622 // 22037
    res as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 5);
        assert_eq!(solve(true, false), 12);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 8622);
        assert_eq!(solve(false, false), 22037);
    }
}