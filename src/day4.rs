use crate::utils::read_challenge_data;
use itertools::Itertools;

/// A board data structure that keeps
/// `numbers` =  numbers of the board
/// `marked` = matrix of bools to see if a number has been marked
/// `last_number` the last number played to calculate the score
#[derive(Debug)]
pub struct Board {
    numbers: [[isize; 5]; 5],
    marked: [[bool; 5]; 5],
    last_number: Option<isize>,
}
impl Board {
    fn new(numbers: [[isize; 5]; 5]) -> Self {
        Self {
            numbers,
            marked: [[false; 5]; 5],
            last_number: None,
        }
    }

    /// Calculate the score of the board as the sum of unmarked numbers
    /// multiplied with the last number played.
    /// If no number has been played returns None
    fn score(&self) -> Option<isize> {
        if let Some(last_number) = self.last_number {
            let mut score = 0;
            for (row_numbers, row_marked) in self.numbers.iter().zip(self.marked) {
                for (elem, mark) in row_numbers.iter().zip(row_marked) {
                    // Add unmarked
                    if !mark {
                        score += elem;
                    }
                }
            }
            return Some(score * last_number);
        }
        None
    }

    /// Checks winning conditions
    fn check_win(&self) -> bool {
        // Check rows
        if self.marked.iter().any(|&row| row.iter().all(|&mark| mark)) {
            return true;
        }
        // Check columns
        if (0..5).any(|j| (0..5).all(|i| self.marked[i][j])) {
            return true;
        }
        false
    }
    /// Marks the first occurence of a given number.
    /// Iterates through rows then columns.
    /// Returns false if the number is not found
    fn mark_number(&mut self, number: isize) -> bool {
        for (row_numbers, row_marked) in self.numbers.iter_mut().zip(self.marked.iter_mut()) {
            for (&mut elem, mark) in row_numbers.iter_mut().zip(row_marked.iter_mut()) {
                if elem == number {
                    *mark = true;
                    self.last_number = Some(number);
                    return true;
                }
            }
        }
        false
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(4, run_example);
    let _buf: String = String::from("");
    // Read winning numbers
    let first_line = input.lines().next().unwrap();
    let winning_numbers: Vec<isize> = first_line
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    // Read 6 lines at a time to account for the empty line
    for lines in input.lines().skip(1).chunks(6).into_iter() {
        let mut numbers = [[0; 5]; 5];
        // Skip the empty line
        for (i, line) in lines.skip(1).enumerate() {
            let line_numbers: Vec<isize> = line
                .trim()
                .split(' ')
                .filter_map(|s| s.parse::<isize>().ok())
                .collect();
            numbers[i] = line_numbers.try_into().unwrap();
        }
        boards.push(Board::new(numbers));
    }

    // Vector of bools to keep track of the winning boards
    let mut winning_boards = vec![false; boards.len()];
    let mut ranking: Vec<usize> = Vec::new();
    'outer: for number in winning_numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            // Only boards that haven't won play
            if !winning_boards[i] {
                board.mark_number(number);
                if board.check_win() {
                    winning_boards[i] = true;
                    ranking.push(i)
                }
            }
            // stop if all boards won
            if winning_boards.iter().all(|&e| e) {
                break 'outer;
            }
        }
    }

    // Print first and last board
    if part1 {
        println!(
            "First board is {} with a score of {}",
            ranking[0] + 1,
            boards[ranking[0]].score().unwrap()
        );
        boards[ranking[0]].score().unwrap() as usize
    } else {
        println!(
            "Last board is {} with a score of {}",
            ranking.last().unwrap() + 1,
            boards[*ranking.last().unwrap()].score().unwrap()
        );
        boards[*ranking.last().unwrap()].score().unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 4512);
        assert_eq!(solve(true, false), 1924);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 22680);
        assert_eq!(solve(false, false), 16168);
    }
}