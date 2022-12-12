use crate::solver;

pub fn solve(input: &Vec<Vec<char>>) {
    println!(
        "Part 2 solution: {}",
        solver::solve(vec![solver::START, 'a'], input)
    );
}
