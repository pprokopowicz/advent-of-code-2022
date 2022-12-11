use crate::monkey::Monkey;
use crate::solver;

pub fn solve(input: &Vec<Monkey>) {
    let mut input = input.clone();

    let output = solver::solve(&mut input, 10000, false);

    println!("Part 2 solution: {}", output);
}
