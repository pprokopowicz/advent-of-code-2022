use std::fs;

pub enum File {
    Day01 = 1,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

fn file_name(file: File) -> String {
    let index = file as u32;
    if index > 9 {
        format!("inputs/day{}.txt", index)
    } else {
        format!("inputs/day0{}.txt", index)
    }
}

pub fn read_file(file: File) -> String {
    fs::read_to_string(file_name(file)).unwrap()
}
