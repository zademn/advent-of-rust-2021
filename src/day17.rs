use std::ops::{Add, AddAssign};

use itertools::Itertools;

use crate::utils::read_challenge_data;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vector2 {
    x: i64,
    y: i64,
}
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Vector2) {
        *self = *self + other;
    }
}
struct Probe {
    position: Vector2,
    velocity: Vector2,
}
impl Probe {
    fn update(&mut self) {
        self.position += self.velocity;
        self.velocity.x -= self.velocity.x.signum() * 1; // signum gets the sign of the number
        self.velocity.y -= 1; // gravity
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]

struct Rectangle {
    top_left_corner: Vector2,
    bottom_right_corner: Vector2,
}
impl Rectangle {
    /// Checks if a projectile is in the area. Border included
    fn contains(&self, point: &Vector2) -> bool {
        let Vector2 { x, y } = *point;
        x >= self.top_left_corner.x
            && x <= self.bottom_right_corner.x
            && y <= self.top_left_corner.y
            && y >= self.bottom_right_corner.y
    }
    /// Checks if a projectile passed the rectangle
    fn passed(&self, probe: &Probe) -> bool {
        (probe.position.y < self.bottom_right_corner.y && probe.velocity.y < 0)
            || (probe.position.x > self.bottom_right_corner.x && probe.velocity.x > 0)
            || (probe.position.x < self.top_left_corner.x && probe.velocity.x < 0)
        //true;
    }
}

/// Checks if a probe will hit the area
fn max_height(probe: Probe, target_area: &Rectangle) -> Option<i64> {
    let mut probe = probe;
    let mut y_max = i64::MIN;
    loop {
        y_max = y_max.max(probe.position.y);
        if target_area.contains(&probe.position) {
            return Some(y_max);
        }
        if target_area.passed(&probe) {
            return None;
        }
        probe.update();
    }
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    // Parsing is pain
    let input = read_challenge_data(17, run_example);
    // Parse fiesta
    let ((x1, x2), (y1, y2)) = input
        .trim_start_matches("target area: x=")
        .splitn(2, ", y=")
        .map(|s| {
            s.trim()
                .splitn(2, "..")
                .map(|s| s.parse().unwrap())
                .tuples::<(i64, i64)>()
                .next()
                .unwrap()
        })
        .tuples::<(_, _)>()
        .next()
        .unwrap();

    let target_area = Rectangle {
        top_left_corner: Vector2 { x: x1, y: y2 },
        bottom_right_corner: Vector2 { x: x2, y: y1 },
    };

    // Parsing is pain
    // let target_area = if run_example {
    //     Rectangle {
    //         top_left_corner: Vector2 { x: 20, y: -5 },
    //         bottom_right_corner: Vector2 { x: 30, y: -10 },
    //     }
    // } else {
    //     Rectangle {
    //         top_left_corner: Vector2 { x: 138, y: -71 },
    //         bottom_right_corner: Vector2 { x: 184, y: -125 },
    //     }
    // };

    let y_limit = target_area.bottom_right_corner.y.abs();

    let mut y_max = 0;
    let mut c = 0;

    // brute force the x values
    for x in -1000..1000 {
        for y in -y_limit..y_limit {
            let probe = Probe {
                position: Vector2 { x: 0, y: 0 },
                velocity: Vector2 { x, y },
            };
            if let Some(possible_max) = max_height(probe, &target_area) {
                y_max = y_max.max(possible_max);
                c += 1;
            }
        }
    }
    if part1 {
        println!("{}", y_max); //7750
        y_max as usize
    } else {
        println!("{}", c); // 4120
        c
    }
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 45);
        assert_eq!(solve(true, false), 112);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 7750);
        assert_eq!(solve(false, false), 4120);
    }
}
