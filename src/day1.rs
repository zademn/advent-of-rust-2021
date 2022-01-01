use crate::utils::read_challenge_data;

/// Pretty straightforward
/// We have the `first: bool` which represents if the first number was introduced
/// We store in `previous` the previous value.
/// We store in `count` the number of times the depth has increased
/// We read a line, parse it,compare and increase the count
/// if it's bigger than the previous

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(1, run_example);

    let mut first = true;
    let mut previous = 0;
    let mut count = 0;

    if part1 {
        for line in input.lines() {
            let depth = line.parse::<u32>().unwrap();
            if first {
                first = false;
            } else if depth > previous {
                count += 1;
            }
            previous = depth;
        }
    } else {
        let depths: Vec<u32> = input
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .collect();

        for ((x, y), z) in depths.iter().zip(&depths[1..]).zip(&depths[2..]) {
            let sliding = x + y + z;
            if first {
                first = false;
            } else if sliding > previous {
                count += 1;
            }
            previous = sliding;
        }
    }

    println!("count: {}", count); // 1759
    count
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 7);
        assert_eq!(solve(true, false), 5);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 1759);
        assert_eq!(solve(false, false), 1805);
    }
}
