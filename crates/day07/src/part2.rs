use crate::parser::Node;
use crate::sizes;

pub fn solve(input: &Node) {
    let total_space = 70000000;
    let available_space = total_space - input.size();
    let min_space = 30000000 - available_space;

    let output = sizes::dir_sizes(input)
        .into_iter()
        .filter(|size| *size >= min_space)
        .min()
        .unwrap();

    println!("Part 2 solution: {}", output);
}
