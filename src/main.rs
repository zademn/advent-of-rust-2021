mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use clap::{App, Arg};


fn challenge(day: u32, part: u32, run_example: bool) {
    match (day, part) {
        (1, 1) => day1::solve(run_example, true),
        (1, 2) => day1::solve(run_example, false),
        (2, 1) => day2::solve(run_example, true),
        (2, 2) => day2::solve(run_example, false),
        (3, 1) => day3::solve(run_example, true),
        (3, 2) => day3::solve(run_example, false),
        (4, 1) => day4::solve(run_example, true),
        (4, 2) => day4::solve(run_example, false),
        (5, 1) => day5::solve(run_example, true),
        (5, 2) => day5::solve(run_example, false),
        (6, 1) => day6::solve(run_example, true),
        (6, 2) => day6::solve(run_example, false),
        (7, 1) => day7::solve(run_example, true),
        (7, 2) => day7::solve(run_example, false),
        (8, 1) => day8::solve(run_example, true),
        (8, 2) => day8::solve(run_example, false),
        (9, 1) => day9::solve(run_example, true),
        (9, 2) => day9::solve(run_example, false),
        (10, 1) => day10::solve(run_example, true),
        (10, 2) => day10::solve(run_example, false),
        (11, 1) => day11::solve(run_example, true),
        (11, 2) => day11::solve(run_example, false),
        (12, 1) => day12::solve(run_example, true),
        (12, 2) => day12::solve(run_example, false),
        (13, 1) => day13::solve(run_example, true),
        (13, 2) => day13::solve(run_example, false),
        (14, 1) => day14::solve(run_example, true),
        (14, 2) => day14::solve(run_example, false),
        (15, 1) => day15::solve(run_example, true),
        (15, 2) => day15::solve(run_example, false),
        (16, 1) => day16::solve(run_example, true),
        (16, 2) => day16::solve(run_example, false),
        (17, 1) => day17::solve(run_example, true),
        (17, 2) => day17::solve(run_example, false),
        (18, 1) => day18::solve(run_example, true),
        (18, 2) => day18::solve(run_example, false),
        (19, 1) => day19::solve(run_example, true),
        (19, 2) => day19::solve(run_example, false),
        (20, 1) => day20::solve(run_example, true),
        (20, 2) => day20::solve(run_example, false),
        (21, 1) => day21::solve(run_example, true),
        (21, 2) => day21::solve(run_example, false),
        (22, 1) => day22::solve(run_example, true),
        (22, 2) => day22::solve(run_example, false),
        (23, 1) => day23::solve(run_example, true),
        (23, 2) => day23::solve(run_example, false),
        (24, 1) => day24::solve(run_example, true),
        (24, 2) => day24::solve(run_example, false),
        (25, 1) => day25::solve(run_example, true),
        (25, 2) => day25::solve(run_example, false),
        _ => 0,
    };
}

fn main() {
    let matches = App::new("Advent of Rust 2021")
        .author("Zademn")
        .arg(
            Arg::with_name("day")
                .help("Day of the challenge")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("part")
                .help("Part of the challenge: 1 or 2")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("example")
                .short("e")
                .help("If the example should be run"),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap().parse().unwrap();
    let part = matches.value_of("part").unwrap().parse().unwrap();
    let run_example = matches.is_present("example");
    //challenge!(day.as_str(), part.as_str());
    challenge(day, part, run_example);
}
