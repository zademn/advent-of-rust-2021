use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_challenge_data;

/// Reverse lookup in a Hashmap<char, char>
fn reverse_lookup(map: &HashMap<char, char>, value: char) -> Option<char> {
    for (&k, &v) in map.iter() {
        if v == value {
            return Some(k);
        }
    }
    None
}

/// 1. Find unique letters by counts
/// `a` -- 8 times
/// `b` -- 6 times -- can find
/// `c` -- 8 times
/// `d` -- 7 times
/// `e` -- 4 times -- can find
/// `f` -- 9 times -- can find
/// `g` -- 7 times
/// 2. Intersection
/// 3. 1 intersected with 7 gives `a`
/// 4. One we know `f` we can find `c` from 1.
/// 5. Once you know `c` you can get 'd' from 4;
/// 6. Once you know 'd' the last one to find is 'g' which we can find from 8
fn create_map(digits: Vec<&str>) -> HashMap<char, char> {
    let mut map: HashMap<char, char> = HashMap::new();

    // Count digit frequency
    let mut count_map: HashMap<char, i32> = HashMap::new();
    let mut seven = "";
    let mut one = "";
    let mut four = "";
    let mut eight = "";
    for digit in digits {
        for c in digit.chars() {
            count_map.insert(c, count_map.get(&c).unwrap_or(&0) + 1);
        }
        // find seven and one
        match digit.len() {
            2 => one = digit,
            3 => seven = digit,
            4 => four = digit,
            7 => eight = digit,
            _ => (),
        };
    }
    for (key, value) in count_map {
        match value {
            4 => map.insert(key, 'e'),
            6 => map.insert(key, 'b'),
            9 => map.insert(key, 'f'),
            _ => None,
        };
    }
    // Find `a`
    for c in seven.chars() {
        if !one.contains(c) {
            map.insert(c, 'a');
            break;
        }
    }
    // Find `c`
    let f = reverse_lookup(&map, 'f').unwrap();
    for c in one.chars() {
        if c != f {
            map.insert(c, 'c');
            break;
        }
    }
    // Find 'd'
    let bcf = [
        reverse_lookup(&map, 'b').unwrap(),
        reverse_lookup(&map, 'c').unwrap(),
        reverse_lookup(&map, 'f').unwrap(),
    ];
    for c in four.chars() {
        if !bcf.contains(&c) {
            map.insert(c, 'd');
            break;
        }
    }
    // Find `g`

    for c in eight.chars() {
        if !map.keys().copied().any(|x| x == c) {
            map.insert(c, 'g');
            break;
        }
    }
    map
}

/// Sort the signal alphabetically and decode it
fn decode_signal_map(signal: &str, map: &HashMap<char, char>) -> Option<isize> {
    let s_decoded: String = signal
        .chars()
        .map(|c| map.get(&c).unwrap())
        .sorted()
        .collect();
    match s_decoded.as_str() {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(9),
        _ => None,
    }
}

/// Decode unique signals based on the length of the signal
fn decode_signal(signal: &str) -> Option<isize> {
    match signal.len() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(8, run_example);

    let mut c = 0;
    for line in input.lines() {
        let s: Vec<&str> = line.split(" | ").collect();
        assert_eq!(s.len(), 2);
        let first_digits: Vec<&str> = s[0].split(' ').collect();
        let last_digits: Vec<&str> = s[1].split(' ').collect();
        // println!("{:?}", first_digits);
        // println!("{:?}", last_digits);

        if part1 {
            for s in last_digits {
                if decode_signal(s).is_some() {
                    c += 1;
                }
            }
        } else {
            // Create map from the first 10 digits given
            let map = create_map(first_digits);
            //println!("{:?}", map);
            // Decode last digits and construct the number
            let mut n = 0;
            for (i, &digit) in last_digits.iter().rev().enumerate() {
                n += 10isize.pow(i as u32) * decode_signal_map(digit, &map).unwrap();
            }
            c += n;
        }
    }
    println!("{}", c); // 239 // 946346
    c as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 26);
        assert_eq!(solve(true, false), 61229);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 239);
        assert_eq!(solve(false, false), 946346);
    }
}
