use reader::{read_file, File};

use crate::instruction::Instruction;

pub fn parse() -> Vec<Instruction> {
    let input = read_file(File::Day10);

    input
        .lines()
        .map(|line| {
            if line == "noop" {
                return Instruction::Noop;
            }

            if line.starts_with("addx") {
                let split = line.split_whitespace().collect::<Vec<&str>>();
                let value = split[1]
                    .parse::<isize>()
                    .expect("addx instruction is missing value");

                return Instruction::Addx(value);
            }

            panic!("Unknown instruction");
        })
        .collect::<Vec<Instruction>>()
}
