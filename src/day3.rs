use crate::utils::read_challenge_data;

/// Straightforward using a frequency vector where we count `1`
/// Then we can calculate the most frequent bit if
/// it appears more than half the number of lines
pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(3, run_example);
    // 111100101100 -> 12 bits
    let num_bits: usize = if run_example { 5 } else { 12 };
    let mut freq_vec = vec![0; num_bits];

    if part1 {
        let mut num_lines = 0;

        for line in input.lines() {
            let diagnostic = line;
            for (i, c) in diagnostic.chars().enumerate() {
                if c == '1' {
                    freq_vec[i] += 1;
                }
            }
            num_lines += 1;
        }

        // println!("res: {:?}", freq_vec);
        // println!("num lines: {}", num_lines);
        let mut gamma = 0;
        let mut epsilon = 0;
        for r in freq_vec.iter() {
            if *r > num_lines / 2 {
                gamma = (gamma << 1) | 1;
                epsilon <<= 1;
            } else {
                gamma <<= 1;
                epsilon = (epsilon << 1) | 1;
            }
        }
        println!("{}", gamma * epsilon); // 845186
        gamma * epsilon
    } else {
        /// Mutates a given bit frequency vector
        /// by adding the bits of a numeber `t`.
        /// MSB is position freq_vec[0];
        fn update_freq_vec(freq_vec: &mut Vec<u32>, t: u32, num_bits: usize) {
            for (i, elem) in freq_vec.iter_mut().enumerate() {
                if (t >> (num_bits - i - 1)) & 1 == 1 {
                    *elem += 1;
                }
            }
        }

        // 111100101100 -> 12 bits
        let mut diagnostics: Vec<u32> = Vec::new(); // we here we keep all the numbers
        for line in input.lines() {
            let diagnostic = line.trim();
            let t = u32::from_str_radix(diagnostic, 2).unwrap();
            diagnostics.push(t);
            update_freq_vec(&mut freq_vec, t, num_bits);
        }
        //println!("{:?}", freq_vec);
        //println!("{:?}", diagnostics);
        let mut vec_o2 = diagnostics.clone();
        let mut freq_o2 = freq_vec.clone();

        for i in 0..num_bits {
            // Save elements of interest
            let l = vec_o2.len() as f32;
            vec_o2.retain(|&elem| {
                (elem >> (num_bits - i - 1)) & 1 == (freq_o2[i] as f32 >= l / 2.) as u32
            });
            // Break if we reached 1 element or less
            if vec_o2.len() <= 1 {
                break;
            }
            freq_o2 = vec![0; num_bits];
            for elem in &vec_o2 {
                update_freq_vec(&mut freq_o2, *elem, num_bits);
            }
        }

        let mut vec_co2 = diagnostics.clone();
        let mut freq_co2 = freq_vec.clone();

        for i in 0..num_bits {
            // Save elements of interest
            let l = vec_co2.len() as f32;
            vec_co2.retain(|&elem| {
                (elem >> (num_bits - i - 1)) & 1 == ((freq_co2[i] as f32) < l / 2.) as u32
            });
            // Break if we reached 1 element or less
            if vec_co2.len() <= 1 {
                break;
            }
            // Update frequency vector
            freq_co2 = vec![0u32; num_bits];
            for elem in &vec_co2 {
                update_freq_vec(&mut freq_co2, *elem, num_bits);
            }
        }
        println!("{}", vec_o2[0] * vec_co2[0]); // 4636702

        (vec_o2[0] * vec_co2[0]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 198);
        assert_eq!(solve(true, false), 230);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 845186);
        assert_eq!(solve(false, false), 4636702);
    }
}
