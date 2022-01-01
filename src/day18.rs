use crate::utils::read_challenge_data;

/// If parent is on idx =>
///     left_child = (2 * idx) + 1
///     left_child = (2 * idx) + 2
struct Tree {
    values: Vec<i64>,
}

impl Tree {
    fn insert(&mut self, value: i64, parent: usize, left: bool) {
        let pos = 2 * parent + if left { 1 } else { 2 };
        self.values.insert(pos, value);
    }
    fn parent_idx(&self, node: usize) -> usize {
        (node - 1) / 2 // rounds down for odds
    }
    fn children_idxs(&self, parent: usize) -> (usize, usize) {
        (2 * parent + 1, 2 * parent + 2)
    }
    fn get(&self, node: usize) -> Option<&i64> {
        self.values.get(node)
    }
}

fn split_number(n: usize) -> (usize, usize) {
    if n & 1 == 1 {
        (n / 2, n / 2 + 1)
    } else {
        (n / 2, n / 2)
    }
}
fn explode() {}

pub fn solve(run_example: bool, _part1: bool) -> usize {
    let _input = read_challenge_data(18, run_example);

    let i: usize = 3;
    println!("{}", (i - 1) / 2);
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
