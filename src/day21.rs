use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::{read_challenge_data, wrap};

struct Die {
    value: usize,
}
impl Die {
    fn roll(&mut self) -> usize {
        self.value = wrap(self.value + 1, 1, 100);
        self.value
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    positions: [usize; 2],
    scores: [usize; 2],
    turn: usize,
}

// Frequency array for each possible sum of 3 dices: [3, 4, 5, 6, 7, 8, 9]
static POSSIBLE_ROLLS: [usize; 7] = [3, 4, 5, 6, 7, 8, 9];
static FREQ_SUM: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

fn dp(state: State, memo: &mut HashMap<State, (usize, usize)>) -> (usize, usize) {
    // Check if we encountered this state before
    if let Some(&res) = memo.get(&state) {
        return res;
    }

    // End conditions
    if state.scores[0] >= 21 {
        return (1, 0);
    }
    if state.scores[1] >= 21 {
        return (0, 1);
    }

    let mut res = (0, 0);
    let current_player = state.turn % 2;

    for (&roll, &f) in POSSIBLE_ROLLS.iter().zip(&FREQ_SUM) {
        // compute new state
        let mut new_state = state;
        new_state.positions[current_player] = wrap(state.positions[current_player] + roll, 1, 10);
        new_state.scores[current_player] += new_state.positions[current_player];
        new_state.turn += 1;
        // Get values for next state
        let new_res = dp(new_state, memo);

        // Multiply by the frequency of hitting the roll
        res.0 += f * new_res.0;
        res.1 += f * new_res.1;
    }
    // Save the state into memo;
    memo.insert(state, res);
    res
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(21, run_example);
    let (mut p1, mut p2) = input
        .lines()
        .map(|s| {
            s.trim().trim_start_matches("Player ")[1..]
                .trim_start_matches(" starting position: ")
                .parse::<usize>()
                .unwrap()
        })
        .tuples()
        .next()
        .unwrap();

    if part1 {
        let (mut score1, mut score2) = (0, 0);
        let mut p1_turn = true;
        let mut ddie = Die { value: 0 };
        let mut total_rolls = 0;

        while score1 < 1000 && score2 < 1000 {
            if p1_turn {
                let (r1, r2, r3) = (ddie.roll(), ddie.roll(), ddie.roll());
                p1 = wrap(p1 + r1 + r2 + r3, 1, 10);
                score1 += p1;
                p1_turn = false;
            } else {
                let (r1, r2, r3) = (ddie.roll(), ddie.roll(), ddie.roll());
                p2 = wrap(p2 + r1 + r2 + r3, 1, 10);
                score2 += p2;
                p1_turn = true;
            }
            total_rolls += 3;
        }
        let c = if score1 >= 1000 {
            score2 * total_rolls
        } else {
            score1 * total_rolls
        };
        println!("{}", c);
        return c;
    }

    // memoization table
    let mut memo = HashMap::new();

    let state = State {
        positions: [p1, p2],
        scores: [0, 0],
        turn: 0,
    };
    let res = dp(state, &mut memo);

    println!("{:?}", res.0.max(res.1));
    res.0.max(res.1)
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 739785);
        assert_eq!(solve(true, false), 444356092776315);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 598416);
        assert_eq!(solve(false, false), 27674034218179);
    }
}
