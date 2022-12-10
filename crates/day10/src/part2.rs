use crate::instruction::Instruction;

const ON: char = '#';
const OFF: char = ' ';

pub fn solve(input: &Vec<Instruction>) {
    let mut x = 1;
    let mut output = "".to_string();

    input.iter().for_each(|instruction| match instruction {
        Instruction::Noop => draw(&x, &mut output),
        Instruction::Addx(value) => {
            draw(&x, &mut output);
            draw(&x, &mut output);
            x += value;
        }
    });

    println!("Part 2 solution:\n{}", output);
}

fn draw(x: &isize, output: &mut String) {
    let current_index = (output.len() % 41) as isize;

    if x == &current_index || (x - 1) == current_index || (x + 1) == current_index {
        output.push(ON);
    } else {
        output.push(OFF);
    }

    if current_index == 39 {
        output.push('\n');
    }
}
