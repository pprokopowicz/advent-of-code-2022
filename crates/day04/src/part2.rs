use crate::parser::Input;
use crate::range_ext::Overlaps;

pub fn solve(input: &Vec<Input>) {
    let sum = input
        .iter()
        .map(|input| {
            if input.lhs.overlaps(&input.rhs) || input.rhs.overlaps(&input.lhs) {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Part 2 solution: {}", sum);
}
