use crate::format_out::format_out;
use crate::parser::Input;

pub fn solve(input: &Input) {
    let mut stacks = input.stacks.clone();

    for instruction in &input.instructions {
        let mut tmp = vec![];
        for _ in 0..instruction.amount {
            tmp.push(stacks[instruction.origin - 1].pop().unwrap());
        }

        for _ in 0..instruction.amount {
            stacks[instruction.destination - 1].push(tmp.pop().unwrap());
        }
    }

    println!("Part 2 solution: {}", format_out(&stacks));
}
