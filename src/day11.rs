use std::collections::HashSet;

use crate::utils::read_challenge_data;

type Point = (usize, usize);

/// Simple struct that keeps the energy level and a flag to mark if the octopus flashed this round
#[derive(Clone, Debug)]
struct DumboOctopus {
    energy: isize,
    has_flashed: bool,
}

impl DumboOctopus {
    fn flash(&mut self) {
        self.energy = 0;
        self.has_flashed = true;
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(11, run_example);

    // Our matrix will be a
    let mut cavern: Vec<Vec<DumboOctopus>> = Vec::new();
    for line in input.lines() {
        let t: Vec<DumboOctopus> = line
            .trim()
            .split("")
            .filter_map(|c| c.parse().ok())
            .map(|energy| DumboOctopus {
                energy,
                has_flashed: false,
            })
            .collect();
        cavern.push(t);
    }
    //println!("{:?}", cavern);

    let n_rows = cavern.len();
    let n_cols = cavern[0].len();

    // For part 1 we will break the step at 100 and for part 2 when all flash
    // To solve this we will use a queue to keep the positions of the octopuses
    // that flashed and finish the step only when the queue is empty.
    // If the queue is not empty we go through the points and update the neighbours
    // and if we find another octopus that flashed we put it in the queue
    let mut c = 0; // for part 1
    let mut step = 0; // for part 2
    loop {
        step += 1;
        let mut queue: HashSet<Point> = HashSet::new(); // Empty queue for each step

        // + 1 all
        for (i, row) in cavern.iter_mut().enumerate() {
            for (j, octopus) in row.iter_mut().enumerate() {
                octopus.energy += 1;
                if octopus.energy > 9 {
                    octopus.flash();
                    c += 1;
                    queue.insert((i, j)); // Add position of the octopuss that flashed to the queue
                }
            }
        }
        // While the queue is not empty we update the octopus' neighbours
        // If we find another one that flashed we put it in the queue
        while !queue.is_empty() {
            //println!("{:?}", queue);
            let mut to_add = HashSet::new(); // for updating the HashSet later
            for point in queue.drain() {
                let (x, y) = point;
                //topleft
                // We check the boundary and we only update if the neighbour hasn't flashed this round
                if x > 0 && y > 0 && !cavern[x - 1][y - 1].has_flashed {
                    cavern[x - 1][y - 1].energy += 1;
                    // If we get to the energy threshhold we flash the octopus and we put it in the queue
                    if cavern[x - 1][y - 1].energy > 9 {
                        cavern[x - 1][y - 1].flash();
                        c += 1;
                        to_add.insert((x - 1, y - 1));
                    }
                }
                // top
                if y > 0 && !cavern[x][y - 1].has_flashed {
                    cavern[x][y - 1].energy += 1;
                    if cavern[x][y - 1].energy > 9 {
                        cavern[x][y - 1].flash();
                        c += 1;
                        to_add.insert((x, y - 1));
                    }
                }
                //top right
                if x < n_rows - 1 && y > 0 && !cavern[x + 1][y - 1].has_flashed {
                    cavern[x + 1][y - 1].energy += 1;
                    if cavern[x + 1][y - 1].energy > 9 {
                        cavern[x + 1][y - 1].flash();
                        c += 1;
                        to_add.insert((x + 1, y - 1));
                    }
                }
                // left
                if x > 0 && !cavern[x - 1][y].has_flashed {
                    cavern[x - 1][y].energy += 1;
                    if cavern[x - 1][y].energy > 9 {
                        cavern[x - 1][y].flash();
                        c += 1;
                        to_add.insert((x - 1, y));
                    }
                }
                // right
                if x < n_rows - 1 && !cavern[x + 1][y].has_flashed {
                    cavern[x + 1][y].energy += 1;
                    if cavern[x + 1][y].energy > 9 {
                        cavern[x + 1][y].flash();
                        c += 1;
                        to_add.insert((x + 1, y));
                    }
                }
                // bottom right
                if x > 0 && y < n_cols - 1 && !cavern[x - 1][y + 1].has_flashed {
                    cavern[x - 1][y + 1].energy += 1;
                    if cavern[x - 1][y + 1].energy > 9 {
                        cavern[x - 1][y + 1].flash();
                        c += 1;
                        to_add.insert((x - 1, y + 1));
                    }
                }
                // bottom
                if y < n_cols - 1 && !cavern[x][y + 1].has_flashed {
                    cavern[x][y + 1].energy += 1;
                    if cavern[x][y + 1].energy > 9 {
                        cavern[x][y + 1].flash();
                        c += 1;
                        to_add.insert((x, y + 1));
                    }
                }
                // bottom left
                if x < n_rows - 1 && y < n_cols - 1 && !cavern[x + 1][y + 1].has_flashed {
                    cavern[x + 1][y + 1].energy += 1;
                    if cavern[x + 1][y + 1].energy > 9 {
                        cavern[x + 1][y + 1].flash();
                        c += 1;
                        to_add.insert((x + 1, y + 1));
                    }
                }
            }
            queue.extend(to_add);
        }
        if part1 && step == 100
            || cavern
                .iter()
                .all(|row| row.iter().all(|octo| octo.has_flashed))
        {
            break;
        }
        // Reset Dumbos has_flashed flag
        for row in cavern.iter_mut() {
            for oct in row {
                oct.has_flashed = false;
                //print!("{}", oct.energy);
            }
            //println!("");
        }
    }
    if part1 {
        println!("{}", c); // 1608
        c
    } else {
        println!("{}", step); // 214
        step 
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 1656);
        assert_eq!(solve(true, false), 195);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 1608);
        assert_eq!(solve(false, false), 214);
    }
}