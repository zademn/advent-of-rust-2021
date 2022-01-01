use num::Integer;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn read_challenge_data(challenge_number: u32, example: bool) -> String {
    let path = if example {
        Path::new("resources").join(format!("day{}example.txt", challenge_number))
    } else {
        Path::new("resources").join(format!("day{}.txt", challenge_number))
    };
    read_challenge_data_path(path)
}

#[allow(unused)]
pub fn read_challenge_data_path(path: PathBuf) -> String {
    let s = match fs::read_to_string(&path) {
        Err(e) => panic!("Couldn't open {}: {}", path.display(), e),
        Ok(f) => f,
    };
    s
    //BufReader::new(file)
}

pub fn wrap<T: Integer + Copy>(x: T, a: T, b: T) -> T {
    assert!(a < b);
    let range = b - a + num::one();
    let res = (x - a) % range;
    if x < num::zero() {
        a + res + num::one()
    } else {
        a + res
    }
}
