use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::utils::read_challenge_data;

/// x, y, z store the bounds min and max
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Cuboid {
    on: bool,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

/// Parse the input
fn parse(s: &str) -> (bool, (isize, isize), (isize, isize), (isize, isize)) {
    let on = s.starts_with("on");
    let s = if on {
        s.trim_start_matches("on ")
    } else {
        s.trim_start_matches("off ")
    };

    let mut t = s.splitn(3, ',').map(|ss| {
        ss[2..]
            .splitn(2, "..")
            .map(|sss| sss.trim().parse::<isize>().unwrap())
            .tuples::<(isize, isize)>()
            .next()
            .unwrap()
    });
    let xs = t.next().unwrap();
    let ys = t.next().unwrap();
    let zs = t.next().unwrap();
    (on, xs, ys, zs)
}

pub fn solve(run_example: bool, part1: bool) -> usize {
    let input = read_challenge_data(22, run_example);

    if part1 {
        // Naive part 1
        // brute force the cubes
        let mut cubes_on = HashSet::new();
        for line in input.lines() {
            let (on, x, y, z) = parse(line);
            println!("{:?}, {:?}, {:?}", x, y, z);
            let x = (x.0.max(-50), x.1.min(50));
            let y = (y.0.max(-50), y.1.min(50));
            let z = (z.0.max(-50), z.1.min(50));
            for xi in x.0..x.1 + 1 {
                for yi in y.0..y.1 + 1 {
                    for zi in z.0..z.1 + 1 {
                        if on {
                            cubes_on.insert((xi, yi, zi));
                        } else {
                            cubes_on.remove(&(xi, yi, zi));
                        }
                    }
                }
            }
        }
        println!("{:?}", cubes_on.len()); //567496
        cubes_on.len() 
    } else {
        // Using coordinate compression
        // Segments: [1, 10], [5, 15], [20, 30] -> [1, 3], [2, 4], [5, 6]
        // Then we do the brute force approach on these
        // https://stackoverflow.com/questions/29528934/coordinate-compression
        let mut cubes_on = HashSet::new();
        let mut cuboids = Vec::new();
        let mut x_coords = Vec::new();
        let mut y_coords = Vec::new();
        let mut z_coords = Vec::new();
        for line in input.lines() {
            let (on, x, y, z) = parse(line);
            cuboids.push(Cuboid { on, x, y, z });
            x_coords.push(x.0);
            x_coords.push(x.1 + 1);
            y_coords.push(y.0);
            y_coords.push(y.1 + 1);
            z_coords.push(z.0);
            z_coords.push(z.1 + 1);
        }

        x_coords.sort_unstable();
        y_coords.sort_unstable();
        z_coords.sort_unstable();

        // Compress -- form a map from the coord to the index in the array
        let x_compressed: HashMap<isize, usize> =
            x_coords.iter().enumerate().map(|(i, &e)| (e, i)).collect();
        let y_compressed: HashMap<isize, usize> =
            y_coords.iter().enumerate().map(|(i, &e)| (e, i)).collect();
        let z_compressed: HashMap<isize, usize> =
            z_coords.iter().enumerate().map(|(i, &e)| (e, i)).collect();

        // Same solution as part 1 but now we use smaller coords
        // so it will finish in reasonable time
        for cuboid in cuboids {
            println!("{:?}", cuboid);
            for xi in *x_compressed.get(&cuboid.x.0).unwrap()
                ..*x_compressed.get(&(cuboid.x.1 + 1)).unwrap()
            {
                for yi in *y_compressed.get(&cuboid.y.0).unwrap()
                    ..*y_compressed.get(&(cuboid.y.1 + 1)).unwrap()
                {
                    for zi in *z_compressed.get(&cuboid.z.0).unwrap()
                        ..*z_compressed.get(&(cuboid.z.1 + 1)).unwrap()
                    {
                        if cuboid.on {
                            cubes_on.insert((xi, yi, zi));
                        } else {
                            cubes_on.remove(&(xi, yi, zi));
                        }
                    }
                }
            }
        }
        let mut res = 0;
        for (xi, yi, zi) in cubes_on {
            res += (x_coords[xi + 1] - x_coords[xi])
                * (y_coords[yi + 1] - y_coords[yi])
                * (z_coords[zi + 1] - z_coords[zi]);
        }
        println!("{}", res); //1355961721298916 -- takes a while
        res as usize
    }
}
#[cfg(test)]
mod tests {
    use super::solve;
    #[test]
    fn test_example() {
        assert_eq!(solve(true, true), 474140);
        assert_eq!(solve(true, false), 2758514936282235);
    }
    #[test]

    fn test_problem() {
        assert_eq!(solve(false, true), 567496);
        assert_eq!(solve(false, false), 1355961721298916);
    }
}