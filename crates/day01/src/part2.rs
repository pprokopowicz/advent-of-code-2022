pub fn solve(values: &mut Vec<usize>) {
    values.sort();

    let len = values.len();

    let solution = values[len - 1] + values[len - 2] + values[len - 3];

    println!("Part 2 solution: {}", solution);
}