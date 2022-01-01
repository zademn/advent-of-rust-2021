use crate::utils::read_challenge_data;

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(6, run_example);
    let fishes: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let days = if part1 { 80 } else { 256 };

    // Naive solution, doesn't work for part 2
    // let mut fishes = fishes;
    // for day in 0..days {
    //     let mut new_fishes = Vec::new();
    //     for (i, fish) in fishes.iter_mut().enumerate() {
    //         if *fish == 0 {
    //             new_fishes.push(8);
    //             *fish = 6;
    //             println!("{}, {}", i, day);
    //         } else {
    //             *fish -= 1;
    //         }
    //     }
    //     fishes.extend(new_fishes);
    //     println!("{:?}", fishes);
    // }
    // println!("{}", fishes.len());

    //println!("{:?}", fishes);

    // For an efficient solution we need to use a fixed length vector,
    // so we are using the days of the week to keep the number of fishes
    // that will multiply that week.
    // We keep a tuple for (current_fishes, new_fishes) since when we multiply
    // we add only current_fishes since new_fishes need 8 days until they multiply
    let mut weekdays: Vec<(u64, u64)> = vec![(0, 0); 7];

    // Initial setup.
    for fish in fishes {
        weekdays[fish].0 += 1;
    }
    for day in 0..days {
        // Add new fishes with 2 day delay
        weekdays[(day + 2) % 7].1 = weekdays[day % 7].0;
        // Update current fishes with the ones added on previous iteration
        weekdays[day % 7].0 += weekdays[day % 7].1;
        // Set new fishes to 0
        weekdays[day % 7].1 = 0;
    }
    let res = weekdays.iter().fold(0, |acc, e| acc + e.0 + e.1);
    println!("{}", res); // 360761 // 1632779838045
    res as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 5934);
        assert_eq!(solve(true, false), 26984457539);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 360761);
        assert_eq!(solve(false, false), 1632779838045);
    }
}