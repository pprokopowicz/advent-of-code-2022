use crate::parser;

pub fn solve() {
    let mut values = parser::parse();
    values.sort();

    let len = values.len();

    let solution = [
        values[len - 1], values[len - 2], values[len - 3]
    ].into_iter().sum::<usize>();

    println!("Part 2 solution: {}", solution);
}