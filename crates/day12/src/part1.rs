use crate::solver;

pub fn solve(input: &Vec<Vec<char>>) {
    println!(
        "Part 1 solution: {}",
        solver::solve(vec![solver::START], input)
    );
}
