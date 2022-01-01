use std::collections::HashSet;

use crate::utils::read_challenge_data;

fn score_illegal(c: char) -> Option<isize> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None,
    }
}
fn match_illegal(open: char, close: char) -> Option<isize> {
    match (open, close) {
        (_, ')') if open != '(' => Some(3),
        (_, ']') if open != '[' => Some(57),
        (_, '}') if open != '{' => Some(1197),
        (_, '>') if open != '<' => Some(25137),
        _ => None,
    }
}

fn score_autocomplete(c: char) -> Option<isize> {
    match c {
        '(' => Some(1),
        '[' => Some(2),
        '{' => Some(3),
        '<' => Some(4),
        _ => None,
    }
}
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(10, run_example);
    let openings: HashSet<char> = ['(', '[', '{', '<'].into();
    let mut s1 = 0; // score for part 1
    let mut s2 = Vec::new(); // score for part 2
    'outer: for line in input.lines() {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            // If it's an opening char push it
            if openings.contains(&c) {
                stack.push(c);
            } else {
                // If the stack is empty add the illegal score and break the loop
                if stack.is_empty() {
                    s1 += score_illegal(c).unwrap();
                    continue 'outer;
                } else {
                    // Else pop the last char (it's an opening char) and check it against the closing char
                    // If they don't match we add it to the score and break the loop
                    if let Some(score) = match_illegal(stack.pop().unwrap(), c) {
                        s1 += score;
                        continue 'outer;
                    }
                    // If everything is fine we continue
                }
            }
        }
        // For part 2 the stack will retain all the unclosed characters
        // We just add up the points according to the formula
        if !part1 {
            let mut t = 0;
            for &c in stack.iter().rev() {
                //print!("{}")
                t = 5 * t + score_autocomplete(c).unwrap() as u64;
            }
            //println!("{}", t);
            s2.push(t);
        }
    }

    if part1 {
        print!("{}", s1); // 39467
        s1 as usize
    } else {
        s2.sort_unstable();
        print!("{}", s2[s2.len() / 2]); // 2380061249
        s2[s2.len() / 2] as usize
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 26397);
        assert_eq!(solve(true, false), 288957);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 394647);
        assert_eq!(solve(false, false), 2380061249);
    }
}