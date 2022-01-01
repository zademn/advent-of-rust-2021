use crate::utils::read_challenge_data;

/// Submarine struct to handle the movement and position
pub struct Submarine {
    x: isize, // horizontal
    y: isize, // vertical, increases with depth
    aim: isize,
}
impl Submarine {
    #[allow(unused)]
    fn new(x: isize, y: isize, aim: isize) -> Self {
        Self { x, y, aim }
    }
    /// Decodes command string into `direction: &str` and `distance: i32`
    fn decode_command(command: &str) -> (&str, isize) {
        let command: Vec<&str> = command.splitn(2, ' ').collect();
        assert_eq!(command.len(), 2, "Invalid command");
        let direction = command[0];
        let distance = command[1].parse().unwrap();

        (direction, distance)
    }
    /// Updates based on part 1, straightforward implementations
    fn update(&mut self, command: &str) {
        let (direction, distance) = Submarine::decode_command(command);
        match direction {
            "forward" => self.x += distance,

            "down" => self.y += distance,
            "up" => self.y -= distance,
            _ => panic!("Invalid direction"),
        }
    }
    /// Updates based on part 2, straightforward implementations
    fn update_aimed(&mut self, command: &str) {
        let (direction, distance) = Submarine::decode_command(command);
        match direction {
            "forward" => {
                self.x += distance;
                self.y += self.aim * distance;
            }
            "down" => self.aim += distance,
            "up" => self.aim -= distance,
            _ => panic!("Invalid direction"),
        }
    }
    /// Getter for x, y
    fn pos(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}
impl Default for Submarine {
    fn default() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(2, run_example);
    let mut submarine = Submarine::default();
    for line in input.lines() {
        let command = line;
        if part1 {
            submarine.update(command);
        } else {
            submarine.update_aimed(command);
        }
    }

    //println!("{:?}", submarine.pos());
    let (x, y) = submarine.pos();
    println!("{}", x * y); // 1990000 // 1975421260
    (x * y) as usize
}

#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 150);
        assert_eq!(solve(true, false), 900);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 1990000);
        assert_eq!(solve(false, false), 1975421260);
    }
}