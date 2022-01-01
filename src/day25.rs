use std::collections::HashSet;

use crate::utils::read_challenge_data;

pub fn solve(run_example: bool, _part1: bool) -> usize {
    let input = read_challenge_data(25, run_example);
    let mut south_cucumbers = HashSet::new();
    let mut east_cucumbers = HashSet::new();

    let mut n_rows = 0;
    let mut n_cols = 0;
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            n_cols = n_cols.max(j + 1);
            n_rows = n_rows.max(i + 1);
            match c {
                '>' => {
                    east_cucumbers.insert((i, j));
                }
                'v' => {
                    south_cucumbers.insert((i, j));
                }
                _ => (),
            }
        }
    }

    // print map
    // for i in 0..n_rows {
    //     for j in 0..n_cols {
    //         if east_cucumbers.contains(&(i, j)) {
    //             print!(">");
    //         } else if south_cucumbers.contains(&(i, j)) {
    //             print!("v");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    let mut c = 0;
    loop {
        c += 1;
        let mut has_moved = false;

        let mut next_east_cucumbers = HashSet::new();
        let mut next_south_cucumbers = HashSet::new();
        for &cucumber in east_cucumbers.iter() {
            // If it's occupied don't move, else insert next position
            let next_pos = (cucumber.0, (cucumber.1 + 1) % n_cols);
            if south_cucumbers.contains(&next_pos) || east_cucumbers.contains(&next_pos) {
                next_east_cucumbers.insert(cucumber);
            } else {
                next_east_cucumbers.insert(next_pos);
                has_moved = true;
            }
        }
        east_cucumbers = next_east_cucumbers;
        for &cucumber in south_cucumbers.iter() {
            // If it's occupied don't move, else insert next position
            let next_pos = ((cucumber.0 + 1) % n_rows, cucumber.1);
            if south_cucumbers.contains(&next_pos) || east_cucumbers.contains(&next_pos) {
                next_south_cucumbers.insert(cucumber);
            } else {
                next_south_cucumbers.insert(next_pos);
                has_moved = true;
            }
        }

        south_cucumbers = next_south_cucumbers;

        if !has_moved {
            break;
        }
    }
    println!("{}", c); // 504
    c
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 58);
        //assert_eq!(solve(true, false), 5);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 504);
        //assert_eq!(solve(false, false), 1805);
    }
}
