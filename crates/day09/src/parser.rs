use reader::{read_file, File};
use std::str::FromStr;

use crate::point::Direction;

pub fn parse() -> Vec<Direction> {
    let input = read_file(File::Day09);

    input
        .lines()
        .flat_map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            let direction = Direction::from_str(split[0]).expect("Unknown movement direction.");
            let value = split[1].parse::<usize>().expect("Value is not an integer");

            (0..value)
                .map(|_| direction.clone())
                .collect::<Vec<Direction>>()
        })
        .collect::<Vec<Direction>>()
}
