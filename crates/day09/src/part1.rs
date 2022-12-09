use crate::point::{Direction, Point};
use crate::rope_sim;

pub fn solve(input: &Vec<Direction>) {
    let mut rope = vec![Point::new(); 2];
    let output = rope_sim::simulate(&mut rope, input);

    println!("Part 1 solution: {}", output);
}