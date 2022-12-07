use crate::parser::Node;
use crate::sizes;

pub fn solve(input: &Node) {
    let output = sizes::dir_sizes(input)
        .into_iter()
        .filter(|size| *size <= 100000)
        .sum::<usize>();

    println!("Part 1 solution: {}", output);
}
