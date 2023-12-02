#!/bin/bash

_usage() {
    echo "Example Usage: $0 YEAR"
    exit 1
}

if [ -z "$1" ]; then
    _usage
fi

if ! [[ $1 =~ ^[0-9]{4}$ ]]; then
    _usage
fi

proj="aoc${1}"

if [ -d "$proj" ]; then
    echo "Directory for year $1 already exists."
    exit 1
fi

echo "creating new project $proj."
cargo new "$proj"

# add local dep
sed -i 's/\[dependencies\]/\[dependencies\]\ncommon = \{path = "..\/common"\}/g' "${proj}/Cargo.toml"

cat >"./${proj}/src/main.rs" << EOL
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::float_arithmetic,
    clippy::modulo_arithmetic,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_lines,
    clippy::similar_names
)]
mod days;

use ${proj}::days::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12, day13,
    day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25,
};
use std::env;
use std::error::Error;

fn get_solver(day: u8) -> fn() {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        21 => day21::solve,
        22 => day22::solve,
        23 => day23::solve,
        24 => day24::solve,
        25 => day25::solve,
        _ => unimplemented!(),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect();
    let days = common::parse_args(&args)?;
    let mut total = 0.0;
    for day in days {
        let day_solver = get_solver(day);
        total += common::time_function(day, day_solver);
    }
    println!("Total runtime: {total:.4} ms");
    return Ok(());
}
EOL

cat >"./${proj}/src/lib.rs" << EOL
pub mod days;
EOL

mkdir -p "${proj}/src/days/data"

for day in `seq -f "%02g" 1 25`; do
    echo "making source file for $day"
    cat >"./${proj}/src/days/day${day}.rs" << EOL
#![allow(clippy::needless_return)]
#[allow(dead_code)]
pub fn solve() {
    println!("day ${day}");
}
EOL
    cat >>"./${proj}/src/days/mod.rs" << EOL
pub mod day${day};
EOL

done