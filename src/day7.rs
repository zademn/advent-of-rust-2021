use crate::utils::read_challenge_data;

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(7, run_example);
    let crab_pos: Vec<isize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    //Straightforward solution:
    // 1. Find posible positions
    // 2. For each position `p` calculate the distances to the position and sum them
    // 3. Find the minimum
    // For part 2 instead of returning the distance return the gauss sum
    // (1 + 2 + 3 + ... + n) = n * (n + 1) / 2
    let min = *crab_pos.iter().min().unwrap();
    let max = *crab_pos.iter().max().unwrap();
    let distances: Vec<isize> = (min..max + 1)
        .map(|p| {
            crab_pos
                .iter()
                .map(|&pos| {
                    let dist = (pos - p).abs();
                    if part1 {
                        dist
                    } else {
                        dist * (dist + 1) / 2
                    }
                })
                .sum()
        })
        .collect();
    let m = *distances.iter().min().unwrap();
    println!("Minimum distance {}", m);
    m as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 37);
        assert_eq!(solve(true, false), 168);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 342534);
        assert_eq!(solve(false, false), 94004208);
    }
}