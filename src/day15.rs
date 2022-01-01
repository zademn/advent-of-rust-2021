use std::collections::{HashMap, HashSet};

use crate::utils::read_challenge_data;

type Position = (usize, usize);
fn neighbours(pos: &Position, size: &Position) -> Vec<Position> {
    let (i, j) = *pos;
    let (rows, cols) = *size;
    let mut res = Vec::new();
    if i >= 1 {
        res.push((i - 1, j));
    }
    if j >= 1 {
        res.push((i, j - 1));
    }
    if i < rows - 1 {
        res.push((i + 1, j));
    }
    if j < cols - 1 {
        res.push((i, j + 1));
    }
    res
}

fn manhattan_distance(pos1: &Position, pos2: &Position) -> u64 {
    ((pos1.0 as isize - pos2.0 as isize).abs() + (pos1.1 as isize - pos2.1 as isize).abs()) as u64
}

#[allow(unused)]
fn euclidean_distance(pos1: &Position, pos2: &Position) -> u64 {
    ((((pos1.0 as f64 - pos2.0 as f64).powi(2) + (pos1.1 as f64 - pos2.1 as f64).powi(2)) as f64)
        .sqrt()) as u64
}
fn reconstruct_path(
    parents: &HashMap<Position, Position>,
    current: &Position,
    start: &Position,
) -> Vec<Position> {
    let mut t = current;
    let mut v: Vec<Position> = Vec::new();
    while t != start {
        v.push(*t);
        t = parents.get(t).unwrap();
    }
    v.push(*start);
    v
}
/// https://en.wikipedia.org/wiki/A*_search_algorithm
fn astar(start: Position, end: Position, map: &[Vec<u64>]) -> Option<Vec<Position>> {
    let map_size = (map.len(), map[0].len());
    let mut open = HashSet::new();
    open.insert(start);
    let mut parents: HashMap<Position, Position> = HashMap::new();
    // g_scores.get(p) = cost of the cheapest path from start to p currently known
    let mut g_scores: HashMap<Position, u64> = HashMap::new();
    g_scores.insert(start, 0);
    // f_scores.get(p) = best guess as to how short a path from start to finish
    // that goes through p is
    let mut f_scores: HashMap<Position, u64> = HashMap::new();
    f_scores.insert(start, manhattan_distance(&start, &end));

    loop {
        // println!("Open: {:?}", open);
        // println!("g_scores: {:?}", g_scores);
        // println!("f_scores: {:?}", f_scores);

        // Get node from open with the lowest f score
        let &current = open
            .iter()
            .min_by(|c1, c2| {
                f_scores
                    .get(c1)
                    .unwrap_or(&u64::MAX)
                    .cmp(f_scores.get(c2).unwrap_or(&u64::MAX))
            })
            .unwrap();
        // If we reached the end break
        //println!("{:?}", current);
        if current == end {
            return Some(reconstruct_path(&parents, &current, &start));
        }
        open.remove(&current);
        for neighbour in neighbours(&current, &map_size) {
            // Distance from start to neighbour through current
            let t_score = *g_scores.get(&current).unwrap_or(&0) + map[neighbour.0][neighbour.1];
            // Path is better, record it.
            if t_score < *g_scores.get(&neighbour).unwrap_or(&u64::MAX) {
                parents.insert(neighbour, current); // set parent to current
                                                    // update scores
                g_scores.insert(neighbour, t_score);
                f_scores.insert(neighbour, t_score + manhattan_distance(&neighbour, &end));

                open.insert(neighbour);
            }
        }
        if open.is_empty() {
            break;
        }
    }
    None
}
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(15, run_example);
    let mut cavern: Vec<Vec<u64>> = Vec::new();
    for line in input.lines() {
        let t: Vec<u64> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        cavern.push(t);
    }

    // Make the cavern 5 times bigger according to the rules
    if !part1 {
        let mut new_cavern = Vec::with_capacity(5 * cavern.len());
        for i in 0..5 {
            for row in &cavern {
                let mut t_extended = Vec::new();
                for j in 0..5 {
                    t_extended.extend(row.iter().map(|e| {
                        let e = *e + i + j;
                        if e <= 9 {
                            e
                        } else {
                            e % 10 + 1
                        }
                    }))
                }
                new_cavern.push(t_extended);
            }
        }
        cavern = new_cavern;
    }
    let end = (cavern.len() - 1, cavern[0].len() - 1);
    //println!("{:?}", end);
    let sp = astar((0, 0), end, &cavern);
    let mut risk = 0;
    if let Some(sp) = sp {
        for p in sp.iter().rev().skip(1) {
            risk += cavern[p.0][p.1];
        }
        println!("{}", risk); //315 // 2998
        
    }
    risk as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 40);
        assert_eq!(solve(true, false), 315);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 790);
        assert_eq!(solve(false, false), 2998);
    }
}