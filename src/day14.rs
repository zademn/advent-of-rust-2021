use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_challenge_data;

fn decode_line(s: &str) -> ((char, char), char) {
    let t: Vec<&str> = s.split(" -> ").collect();
    let (c1, c2) = t[0].chars().tuples().next().unwrap();
    let c3 = t[1].chars().next().unwrap();

    ((c1, c2), c3)
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(14, run_example);
    let mut lines = input.lines();
    let template = lines.next().unwrap().to_string();
    let mut rules = HashMap::new();
    lines.next();
    for line in lines {
        let ((c1, c2), c3) = decode_line(line);
        rules.insert((c1, c2), c3);
    }
    let mut pair_freq: HashMap<(char, char), u64> = HashMap::new();
    let mut char_freq = HashMap::new();

    let steps = if part1 { 10 } else { 40 };

    // Naive solution -- works for part 1
    // let mut old_polymer = template.clone();
    // for _ in 0..steps {
    //     let mut new_polymer = old_polymer.clone();
    //     let mut j = 1;
    //     for (c1, c2) in old_polymer.chars().zip(old_polymer.chars().skip(1)) {
    //         if let Some(v) = rules.get(&(c1, c2)) {
    //             new_polymer.insert(j, *v);
    //             j += 1;
    //         }
    //         j += 1
    //     }
    //     //println!("{}", new_polymer);
    //     old_polymer = new_polymer;
    // }
    // for c in old_polymer.chars() {
    //     let f = char_freq.entry(c).or_insert(0);
    //     *f += 1;
    // }

    // We count each pair frequency.
    // First we count the frequencies in the template
    // Then we go through each pair in the hashmap and
    // 1. we lose the pair -> subtract the number of occurences
    // 2. we create 2 other pairs -> update them by the number of ooccurences of the found pair
    for (c1, c2) in template.chars().zip(template.chars().skip(1)) {
        pair_freq.insert((c1, c2), 1);
    }

    for _ in 0..steps {
        let mut new_pair_freq = pair_freq.clone();
        for ((c1, c2), f) in pair_freq {
            if let Some(&c3) = rules.get(&(c1, c2)) {
                // set the found pair to 0 and get the frequency
                let e = new_pair_freq.entry((c1, c2)).or_insert(0);
                *e -= f;
                let e = new_pair_freq.entry((c1, c3)).or_insert(0);
                *e += f;
                let e = new_pair_freq.entry((c3, c2)).or_insert(0);
                *e += f;
            }
        }
        pair_freq = new_pair_freq;
    }

    for ((c1, _), f) in pair_freq.iter() {
        let f_ = char_freq.entry(c1).or_insert(0);
        *f_ += *f;
    }
    let last_char = template.chars().last().unwrap();
    let f_ = char_freq.entry(&last_char).or_insert(0);
    *f_ += 1;

    println!("{}", template);
    println!("{:?}", char_freq);
    let c_max = char_freq.values().max().unwrap();
    let c_min = char_freq.values().min().unwrap();
    println!("{}", c_max - c_min);
    (c_max - c_min) as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 1588);
        assert_eq!(solve(true, false), 2188189693529);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 3284);
        assert_eq!(solve(false, false), 4302675529689);
    }
}
