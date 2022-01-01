use std::collections::HashMap;

use crate::utils::read_challenge_data;

/// Magic numbers from the algorithm
const MAGIC_ADD_X: [isize; 14] = [11, 13, 11, 10, -3, -4, 12, -8, -3, -12, 14, -6, 11, -12];
const MAGIC_ADD_Y: [isize; 14] = [14, 8, 4, 10, 14, 10, 4, 14, 1, 6, 0, 9, 13, 12];
const MAGIC_DIV_Z: [isize; 14] = [1, 1, 1, 1, 26, 26, 1, 26, 26, 26, 1, 26, 1, 26];

/// Evaluate an operand to be
#[allow(unused)]
fn eval_operand(operand: &str, registers: &HashMap<char, isize>) -> isize {
    if let Some(v) = registers.get(&operand.chars().next().unwrap()) {
        *v
    } else {
        operand.parse::<isize>().unwrap()
    }
}
pub fn solve(run_example: bool, part1: bool) -> usize {
    let _input = read_challenge_data(24, run_example);

    // Failed attempt to parse instructions and execute them
    // let mut number = 100_000_000_000_000u64.to_string();
    // loop {
    //     number = (number.parse::<u64>().unwrap() - 1).to_string();
    //     //println!("{}", number);
    //     if number.chars().any(|c| c == '0') {
    //         continue;
    //     }
    //     let mut input_iter = number.chars().map(|c| c.to_digit(10).unwrap() as isize);
    //     let mut registers: HashMap<char, isize> =
    //         HashMap::from([('x', 0), ('y', 0), ('z', 0), ('w', 0)]);

    //     for line in input.lines() {
    //         let instruction = &line[..3];
    //         let operands = &line[4..];

    //         match instruction {
    //             "inp" => {
    //                 let o = operands.chars().next().unwrap();
    //                 registers.insert(o, input_iter.next().unwrap());
    //             }
    //             "add" => {
    //                 let o1 = operands.chars().next().unwrap();
    //                 let o2v = eval_operand(&operands[2..], &registers);
    //                 *registers.get_mut(&o1).unwrap() += o2v;
    //             }
    //             "mul" => {
    //                 let o1 = operands.chars().next().unwrap();
    //                 let o2v = eval_operand(&operands[2..], &registers);

    //                 *registers.get_mut(&o1).unwrap() *= o2v;
    //             }
    //             "div" => {
    //                 let o1 = operands.chars().next().unwrap();
    //                 let o2v = eval_operand(&operands[2..], &registers);

    //                 *registers.get_mut(&o1).unwrap() /= o2v;
    //             }
    //             "mod" => {
    //                 let o1 = operands.chars().next().unwrap();
    //                 let o2v = eval_operand(&operands[2..], &registers);
    //                 *registers.get_mut(&o1).unwrap() %= o2v;
    //             }
    //             "eql" => {
    //                 let o1 = operands.chars().next().unwrap();
    //                 let o2v = eval_operand(&operands[2..], &registers);

    //                 *registers.get_mut(&o1).unwrap() =
    //                     (*registers.get(&o1).unwrap() == o2v) as isize;
    //             }
    //             _ => {
    //                 panic!("invalid instruction");
    //             }
    //         }
    //     }
    //     //println!("{:?}", number);
    //     if *registers.get(&'z').unwrap() == 0 {
    //         println!("{:?}", number);
    //         println!("{:?}", registers);
    //         break;
    //     }
    // }

    // Failed attempt to decode the program and brute force it.
    // Maybe some optimizations would've cracked it tho
    // let mut number = 100_000_000_000_000u64;
    // loop {
    //     number = number - 1;
    //     let number_string = number.to_string();
    //     //println!("{}", number);
    //     if number_string.chars().any(|c| c == '0') {
    //         continue;
    //     }
    //     let number_iter = number_string
    //         .chars()
    //         .map(|c| c.to_digit(10).unwrap() as isize);
    //     let mut z = 0;
    //     for (i, w) in number_iter.enumerate() {
    //         let start_z = z;
    //         let mut x = z % 26;
    //         z = z / MAGIC_DIV_Z[i];
    //         x = x + MAGIC_ADD_X[i];
    //         // if x = w => x is 0 => z will stay constant
    //         // if w is -MAGIC_ADD_Y[i] => z will stay constant
    //         x = ((x == w) as isize == 0) as isize;
    //         z = z * (25 * x + 1);
    //         z = z + x * (w + MAGIC_ADD_Y[i]);
    //     }
    //     if (number - 1111111111) % 10000000 == 0 {
    //         println!("{}", number);
    //     }
    //     if z == 0 {
    //         println!("found: {}", number);
    //         break;
    //     }
    // }

    // I looked at the code and tried to see what it does. Except z, none of the other registers persist between digits
    // The idea is to keep z = 0 since it's the only value that persists
    // z stays constant if
    //  - the input w to be equal to  z % 26 + MAGIC_ADD_X[i]
    //    since then x will be 0 and z will stay constant
    // otherwise z will get multiplied by 26 and we'll add w + MAGIC_ADD_Y[i] to it => z will grow
    //
    // Another thing to notice is that sometimes we divide z by 26 and coincides with x being negative
    // This is the only way to get z down so we want to abuse this.
    // This happens 7 times out of 14 so we only have to check 7 digits

    let mut w = [0; 14];
    let mut number = if part1 { 10_000_000 } else { 1_000_000 };
    'outer: loop {
        // We use number to go over possible 7-digit combinations.
        // This is a substitute for a cartesian product of 7 numbers
        number -= if part1 { 1 } else { -1 };
        let number_string = number.to_string();
        // Check for 0's
        if number_string.chars().any(|c| c == '0') {
            continue;
        }
        // This generates possible digits to put into our w
        let mut possible_digits = number_string
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize);

        let mut z = 0;

        // Iterate through chars
        for i in 0..14 {
            // The case where we bring z down. Here we want to focus on computing w
            let mut x = z % 26;
            z /= MAGIC_DIV_Z[i];
            x += MAGIC_ADD_X[i]; // the number we want to compare to

            // If we are in the case where we divide by 26
            if MAGIC_DIV_Z[i] == 26 {
                // Check if the number we want to compare to is a digit
                if (1..10).contains(&x) {
                    w[i] = x;
                } else {
                    // If we don't find a digit start over with a new combination of digits
                    continue 'outer;
                }
            } else {
                // If we are not in the case just add the next number from the combination
                w[i] = possible_digits.next().unwrap();
            }

            x = ((x == w[i]) as isize == 0) as isize;
            z *= 25 * x + 1;
            z += x * (w[i] + MAGIC_ADD_Y[i]);
        }
        if z == 0 {
            let res = w
                .into_iter()
                .map(|s| s.to_string())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            println!("found: {}", res); //74929995999389
            return res;
        } else {
            println!("{:?}", w);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::solve;
    // #[test]
    // fn test_example() {
    //     assert_eq!(solve(true, true), 7);
    //     assert_eq!(solve(true, false), 5);
    // }
    #[test]
    fn test_problem() {
        assert_eq!(solve(false, true), 74929995999389);
        assert_eq!(solve(false, false), 11118151637112);
    }
}
