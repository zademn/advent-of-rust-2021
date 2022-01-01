use crate::utils::read_challenge_data;

pub fn solve(run_example: bool, _part1: bool) -> usize {
    let _input = read_challenge_data(23, run_example);
    0
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 0);
        assert_eq!(solve(true, false), 0);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 0);
        assert_eq!(solve(false, false), 0);
    }
}