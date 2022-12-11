use crate::monkey::Monkey;
use crate::solver;

pub fn solve(input: &Vec<Monkey>) {
    let mut input = input.clone();

    let output = solver::solve(&mut input, 20, true);

    println!("Part 1 solution: {}", output);
}
