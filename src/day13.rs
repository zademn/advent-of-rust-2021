use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_challenge_data;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    /// Flips along the given axis identified by coordinate and direction
    /// If the point is on the good side doesn't flip
    fn flip(&mut self, v: usize, dir: Direction) {
        match dir {
            Direction::Horizontal if self.x > v => self.x = v - (self.x - v),
            Direction::Vertical if self.y > v => self.y = v - (self.y - v),
            _ => (),
        }
    }
}

/// Decode the command into direction and line
fn decode_command(s: &str) -> (usize, Direction) {
    //let t: Vec<&str> = s[11..].split('=').collect();
    let (dir, v) = s
        .trim_start_matches("fold along ")
        .split('=')
        .tuples()
        .next()
        .unwrap();
    let v: usize = v.parse().unwrap();
    let dir = if dir == "x" {
        Direction::Horizontal
    } else {
        Direction::Vertical
    };
    (v, dir)
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(13, run_example);
    let mut points: HashSet<Point> = HashSet::new();
    let mut lines = input.lines();
    // Read until the newline
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        let (x, y) = line
            .trim()
            .splitn(2, ',')
            .map(|s| s.parse().unwrap())
            .tuples()
            .next()
            .unwrap();
        let p = Point { x, y };
        points.insert(p);
    }

    for line in lines {
        let (v, dir) = decode_command(line);
        let mut to_add = HashSet::new();
        for mut p in points.drain() {
            p.flip(v, dir);
            to_add.insert(p);
        }
        points.extend(to_add);
        // Break after the first line for part 1
        if part1 {
            println!("{:?}", points.len()); // 678
            return points.len();
        }
    }
    if !part1 {
        let mut cols = 0;
        let mut rows = 0;
        for p in &points {
            cols = cols.max(p.x + 1);
            rows = rows.max(p.y + 1);
        }
        //println!("{}, {}", rows, cols);
        let mut t = vec![vec!['.'; cols]; rows];
        for p in points {
            t[p.y][p.x] = '#';
        }
        for row in t {
            println!("{}", row.iter().collect::<String>());
        } // ECFHLHZE
    }
    0
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 17);
        assert_eq!(solve(true, false), 0);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 678);
        assert_eq!(solve(false, false), 0);
    }
}
