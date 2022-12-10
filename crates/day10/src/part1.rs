use crate::instruction::Instruction;

pub fn solve(input: &Vec<Instruction>) {
    let mut x = 1;
    let mut cycle = 20;
    let mut current_cycle = 0;
    let mut sum = 0;

    input.iter().for_each(|instruction| {
        if current_cycle + instruction.cycles() >= cycle {
            sum += x * cycle;
            cycle += 40;
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(value) => x += value,
        }

        current_cycle += instruction.cycles();
    });

    println!("Part 1 solution: {}", sum);
}
