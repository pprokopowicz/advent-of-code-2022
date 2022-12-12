use crate::solver;

pub fn solve(input: &Vec<String>) {
    println!(
        "Part 2 solution: {}",
        solver::solve(vec![solver::START, 'a'], input)
    );
}
