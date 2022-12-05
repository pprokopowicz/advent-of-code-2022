use crate::format_out::format_out;
use crate::parser::Input;

pub fn solve(input: &Input) {
    let mut stacks = input.stacks.clone();

    for instruction in &input.instructions {
        for _ in 0..instruction.amount {
            let c = stacks[instruction.origin - 1].pop().unwrap();
            stacks[instruction.destination - 1].push(c);
        }
    }

    println!("Part 1 solution: {}", format_out(&stacks));
}
