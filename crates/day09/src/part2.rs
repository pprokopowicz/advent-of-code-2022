use std::collections::HashSet;

use crate::point::{Direction, Point};
use crate::shift::shift_tail_if_needed;

pub fn solve(input: &Vec<Direction>) {
    let mut knots = (0..10).map(|_| Point::new()).collect::<Vec<Point>>();
    let last_index = knots.len() - 1;
    let mut set = HashSet::new();

    set.insert(knots[last_index].clone());

    input.iter().for_each(|direction| {
        knots[0].shift(direction);

        (1..=last_index).for_each(|i| {
            let mut clone = knots[i].clone();
            shift_tail_if_needed(&knots[i - 1], &mut clone);
            knots[i] = clone;
        });

        set.insert(knots[last_index].clone());
    });

    println!("Part 2 solution: {}", set.len());
}
