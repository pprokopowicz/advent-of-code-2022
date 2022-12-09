use std::collections::HashSet;

use crate::point::{Direction, Point};
use crate::shift::shift_tail_if_needed;

pub fn solve(input: &Vec<Direction>) {
    let mut head = Point::new();
    let mut tail = Point::new();
    let mut set = HashSet::new();

    set.insert(tail.clone());

    input.iter().for_each(|direction| {
        head.shift(direction);
        shift_tail_if_needed(&head, &mut tail);
        set.insert(tail.clone());
    });

    println!("Part 1 solution: {}", set.len());
}
