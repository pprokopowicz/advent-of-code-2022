use crate::parser::Input;
use crate::range_ext::ContainsRange;

pub fn solve(input: &Vec<Input>) {
    let sum = input
        .iter()
        .map(|input| {
            if input.lhs.contains_range(&input.rhs) || input.rhs.contains_range(&input.lhs) {
                1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", sum);
}
