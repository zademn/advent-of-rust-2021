use std::collections::HashSet;

use crate::utils::read_challenge_data;
fn bounds(light_pixels: &HashSet<(isize, isize)>) -> (isize, isize, isize, isize) {
    let top = light_pixels.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let bottom = light_pixels.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let left = light_pixels.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let right = light_pixels.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    (top, bottom, left, right)
}

fn generate_new_image(
    light_pixels: &HashSet<(isize, isize)>,
    algorithm: &str,
    flash: bool,
    bounds: (isize, isize, isize, isize),
) -> HashSet<(isize, isize)> {
    let mut new_light_pixels = HashSet::new();
    let (top, bottom, left, right) = bounds;

    println!("{:?}", (top, bottom, left, right));
    for i in top - 1..bottom + 2 {
        for j in left - 1..right + 2 {
            let mut p = 0;
            for di in [-1, 0, 1] {
                for dj in [-1, 0, 1] {
                    p = (p << 1)
                        + if light_pixels.contains(&(i + di, j + dj)) == flash {
                            1
                        } else {
                            0
                        };
                }
            }
            let new_pixel = algorithm.chars().nth(p).unwrap();
            if (new_pixel == '#') != flash {
                new_light_pixels.insert((i, j));
            }
        }
    }

    new_light_pixels
}

#[allow(unused)]
fn imshow(light_pixels: &HashSet<(isize, isize)>) {
    let (top, bottom, left, right) = bounds(light_pixels);
    for i in top..bottom + 1 {
        for j in left..right + 1 {
            if light_pixels.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(20, run_example);
    let mut lines = input.lines();
    let algorithm = lines.next().unwrap().trim();
    lines.next();

    // Save only the light pixels in a set
    let mut light_pixels: HashSet<(isize, isize)> = HashSet::new();
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                light_pixels.insert((i as isize, j as isize));
            }
        }
    }

    let steps = if part1 { 2 } else { 50 };

    // Get the current bounds
    let mut bounds = bounds(&light_pixels);

    for step in 0..steps {
        // flash will tell us if the background will be all lit up or not.
        // This will be used only if algorithm[0] == '#' and algorithm[511] == '.'
        let flash = if algorithm.starts_with('#')
            && algorithm.chars().nth(511).unwrap() == '.'
        {
            step % 2 == 0
        } else {
            false
        };
        light_pixels = generate_new_image(&light_pixels, algorithm, flash, bounds);
        // Increase in each direction by one
        bounds.0 -= 1;
        bounds.1 += 1;
        bounds.2 -= 1;
        bounds.3 += 1;
    }
    println!("{:?}", light_pixels.len());
    light_pixels.len()
}
#[cfg(test)]
mod tests {
    use super::solve;
    // #[test]
    // fn test_example() {
    //     //assert_eq!(solve(true, true), 35);
    //     assert_eq!(solve(true, false), 3351);
    // }
    #[test]
    fn test_problem() {
        assert_eq!(solve(false, true), 5339);
        assert_eq!(solve(false, false), 18395);
    }
}
