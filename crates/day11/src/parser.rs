use reader::{read_file, File};

use crate::monkey::{Monkey, Operation};

pub fn parse() -> Vec<Monkey> {
    let input = read_file(File::Day11);

    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|chunk| Monkey {
            items: parse_items(chunk[1]),
            operation: parse_operation(chunk[2]),
            test_number: parse_number(chunk[3]),
            test_success: parse_number(chunk[4]),
            test_failure: parse_number(chunk[5]),
            num_inspected: 0,
        })
        .collect::<Vec<Monkey>>()
}

fn parse_items(input: &str) -> Vec<usize> {
    input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|num| num.parse::<usize>().expect("Item must be a number"))
        .collect::<Vec<usize>>()
}

fn parse_operation(input: &str) -> Operation {
    let mut split = input.split_whitespace();

    let operation = split.nth(4).unwrap();

    if operation == "*" {
        let last = split.last().unwrap();

        if last == "old" {
            return Operation::Square;
        }

        return Operation::Multiply(last.parse::<usize>().expect("Must be a number"));
    } else if operation == "+" {
        let num = split
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Must be a number");
        return Operation::Add(num);
    } else {
        panic!("Unsupported operation")
    }
}

fn parse_number(input: &str) -> usize {
    input
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .expect("Divisor must be a number")
}
