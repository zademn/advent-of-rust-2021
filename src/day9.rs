use std::collections::HashSet;

use crate::utils::read_challenge_data;

type Point = (usize, usize);

/// Struct that represents the Basin.
/// `low point` -- we keep the local minimum
/// `number` -- represents the idx in the Cave's Vec<Basin>
/// `points` -- The points contained in the basin
#[derive(Clone, Debug)]
struct Basin {
    number: usize,
    low_point: Point,
    points: HashSet<Point>,
}

/// Struct that represents the cave.
/// `heightmap` -- given matrix of heights
/// `basins` -- A vector of basins in the cave.
///     Starts empty and fills after calling `find_basins`
/// `rows, cols` number of rows and cols of the heightmap
struct Cave {
    heightmap: Vec<Vec<isize>>,
    basins: Vec<Basin>,
    rows: usize,
    cols: usize,
}

impl Cave {
    /// Creates a new Cave from the heightmap
    fn new(heightmap: Vec<Vec<isize>>) -> Self {
        let rows = heightmap.len();
        let cols = heightmap[0].len();
        let basins = Vec::new();
        Self {
            heightmap,
            basins,
            rows,
            cols,
        }
    }

    /// Check if it's a local minimum. Also takes care of boundaries
    fn is_local_minimum(&self, pos: Point) -> bool {
        let (i, j) = pos;
        let elem = self.heightmap[i][j];
        if i >= 1 && elem >= self.heightmap[i - 1][j] {
            return false;
        }
        if i < self.rows - 1 && elem >= self.heightmap[i + 1][j] {
            return false;
        }
        if j >= 1 && elem >= self.heightmap[i][j - 1] {
            return false;
        }
        if j < self.cols - 1 && elem >= self.heightmap[i][j + 1] {
            return false;
        }
        true
    }

    /// Naive way to get all low points
    fn low_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.is_local_minimum((i, j)) {
                    points.push((i, j));
                }
            }
        }
        points
    }

    /// Try to roll for each point.
    fn find_basins(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                // Skip `9`
                if self.heightmap[i][j] == 9 {
                    continue;
                }
                self.roll((i, j));
            }
        }
    }

    /// Roll from a starting point and update cave with findings.
    /// Updates basins with new points if we hit a basin
    /// Adds a new basin if we hit a local minimum
    fn roll(&mut self, start_point: Point) -> Point {
        // If current point is in a basin return the lm of the basin
        for basin in self.basins.iter() {
            if basin.points.contains(&start_point) {
                return basin.low_point;
            }
        }
        // Point that we will roll.
        let mut low_point = start_point;
        // Keep track of points that we hit
        let mut points = HashSet::new();
        points.insert(start_point);
        // If we hit a basin we stop and save its number here
        let mut basin_number_op = None;
        // While we roll down
        'outer: while let Some(t) = self.next_point(low_point) {
            // Check if we hit a basin
            for basin in self.basins.iter() {
                if basin.points.contains(&t) {
                    // Keep the index to it
                    basin_number_op = Some(basin.number);
                    break 'outer;
                }
            }
            low_point = t;
            points.insert(t);
        }
        // If we hit something add the points to it. Else add the basin
        if let Some(basin_number) = basin_number_op {
            self.basins[basin_number].points.extend(points);
            low_point = self.basins[basin_number].low_point;
        } else {
            let basin = Basin {
                low_point,
                number: self.basins.len(),
                points,
            };
            self.basins.push(basin);
        }
        low_point
    }

    /// Try to get the next point. Returns None if it's a local minimum
    fn next_point(&self, current_point: Point) -> Option<Point> {
        let (i, j) = current_point;
        let mut elem = self.heightmap[i][j];
        let mut next_point = None;
        if i >= 1 && elem > self.heightmap[i - 1][j] {
            next_point = Some((i - 1, j));
            elem = self.heightmap[i - 1][j];
        }
        if i < self.rows - 1 && elem > self.heightmap[i + 1][j] {
            next_point = Some((i + 1, j));
            elem = self.heightmap[i + 1][j];
        }
        if j >= 1 && elem > self.heightmap[i][j - 1] {
            next_point = Some((i, j - 1));
            elem = self.heightmap[i][j - 1];
        }
        if j < self.cols - 1 && elem > self.heightmap[i][j + 1] {
            next_point = Some((i, j + 1));
        }
        next_point
    }
}
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(9, run_example);

    let mut heightmap: Vec<Vec<isize>> = vec![];
    for line in input.lines() {
        let l: Vec<isize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();
        heightmap.push(l);
    }
    let mut cave = Cave::new(heightmap);

    let res = if part1 {
        // Get the low points coords and sum them up
        let points = cave.low_points();
        points.iter().map(|(i, j)| 1 + cave.heightmap[*i][*j]).sum()
    } else {
        // Find the basins
        cave.find_basins();
        // Sort the basins by their length
        cave.basins
            .sort_by(|a, b| b.points.len().cmp(&a.points.len()));
        // Take the top 3 and multiply them
        cave.basins[..3]
            .iter()
            .fold(1, |acc, basin| acc * basin.points.len()) as isize
    };
    println!("{}", res);
    res as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 15);
        assert_eq!(solve(true, false), 1134);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 444);
        assert_eq!(solve(false, false), 1168440);
    }
}
