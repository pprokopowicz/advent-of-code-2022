use crate::parser;

pub fn solve() {
    let max = parser::parse().into_iter().max().unwrap();

    println!("Part 1 solution: {}", max);
}
