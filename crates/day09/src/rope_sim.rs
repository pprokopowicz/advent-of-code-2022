use std::collections::HashSet;

use crate::point::{Direction, Point};

pub fn simulate(rope: &mut Vec<Point>, input: &Vec<Direction>) -> usize {
    let last_index = rope.len() - 1;
    let mut set = HashSet::new();

    set.insert(rope[last_index].clone());

    input.iter().for_each(|direction| {
        rope[0].shift(direction);

        (1..=last_index).for_each(|i| {
            let mut clone = rope[i].clone();
            shift_tail_if_needed(&rope[i - 1], &mut clone);
            rope[i] = clone;
        });

        set.insert(rope[last_index].clone());
    });

    set.len()
}

fn shift_tail_if_needed(head: &Point, tail: &mut Point) {
    if head.x == tail.x && (head.y - tail.y).abs() > 1 {
        shift_tail_vertically(head, tail);
    } else if head.y == tail.y && (head.x - tail.x).abs() > 1 {
        shift_tail_horizontally(head, tail);
    } else if (head.x - tail.x).abs() == 1 && (head.y - tail.y).abs() > 1 {
        tail.x = head.x;
        shift_tail_vertically(head, tail);
    } else if (head.y - tail.y).abs() == 1 && (head.x - tail.x).abs() > 1 {
        tail.y = head.y;
        shift_tail_horizontally(head, tail);
    } else if (head.y - tail.y).abs() == 2 && (head.x - tail.x).abs() == 2 {
        shift_tail_vertically(head, tail);
        shift_tail_horizontally(head, tail);
    }
}

fn shift_tail_horizontally(head: &Point, tail: &mut Point) {
    if head.x > tail.x {
        tail.x = tail.x + (head.x - tail.x - 1);
    } else {
        tail.x = tail.x + (head.x - tail.x + 1);
    }
}

fn shift_tail_vertically(head: &Point, tail: &mut Point) {
    if head.y > tail.y {
        tail.y = tail.y + (head.y - tail.y - 1);
    } else {
        tail.y = tail.y + (head.y - tail.y + 1);
    }
}
