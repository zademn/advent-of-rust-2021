use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign},
};

use itertools::Itertools;

use crate::utils::read_challenge_data;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn manhattan_distance(&self, other: &Vector3) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn euclidean_distance_squared(&self, other: &Vector3) -> i64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
    }
}
impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = *self + other;
    }
}

pub fn solve(run_example: bool, _part1: bool) -> usize {
    let input = read_challenge_data(19, run_example);

    let mut scanners: HashMap<String, Vec<Vector3>> = HashMap::new();
    let mut current_scanner = None;
    for line in input.lines() {
        if line.is_empty() || line == "\n" || line == "\r\n" {
            continue;
        } else if line.starts_with("--") {
            let scanner_name = line
                .trim()
                .trim_start_matches("--- ")
                .trim_end_matches(" ---")
                .to_string();
            scanners.insert(scanner_name, Vec::new());
            current_scanner = Some(line.to_string());
        } else if let Some(sc) = &current_scanner {
            let (x, y, z) = line
                .trim()
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .tuples::<(i64, i64, i64)>()
                .next()
                .unwrap();
            let v = Vector3 { x, y, z };
            scanners.entry(sc.clone()).and_modify(|e| e.push(v));
        }
    }

    //println!("{:?}", scanners);

    let mut beacons_distances = HashSet::new();
    for (scanner1, beacons1) in &scanners {
        for (scanner2, beacons2) in &scanners {
            if scanner1 != scanner2 {
                for beacon1 in beacons1 {
                    for beacon2 in beacons2 {
                        let d = beacon1.euclidean_distance_squared(beacon2);
                        beacons_distances.insert(d);
                    }
                }
            }
        }
    }
    //println!("{:?}", beacons_distances);
    println!("{}", beacons_distances.len());

    0
}
#[cfg(test)]
mod tests {
    
    // #[test]
    // fn test_example() {
    //     assert_eq!(solve(true, true), 0);
    //     assert_eq!(solve(true, false), 0);
    // }
    // #[test]

    // fn test_problem() {
    //     assert_eq!(solve(false, true), 0);
    //     assert_eq!(solve(false, false), 0);
    // }
}
