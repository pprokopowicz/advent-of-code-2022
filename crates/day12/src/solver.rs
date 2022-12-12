use pathfinding::prelude::bfs;
use std::cmp::min;

use crate::point::Point;

pub const START: char = 'S';
const END: char = 'E';

pub fn solve(starting_char: Vec<char>, input: &Vec<String>) -> usize {
    let end = &find(END, input)[0];

    starting_char
        .iter()
        .flat_map(|c| find(*c, input))
        .filter_map(|point| {
            Some(
                bfs(
                    &point,
                    |point| available_successors(&point, input),
                    |point| point == end,
                )?
                .len(),
            )
        })
        .reduce(|shortest, path| min(shortest, path))
        .expect("No path found")
        - 1
}

fn find(c: char, input: &Vec<String>) -> Vec<Point> {
    input
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            let index = line.find(c);

            match index {
                None => None,
                Some(x) => Some(Point::new(x, y)),
            }
        })
        .collect()
}

fn all_successors(point: &Point, input: &Vec<String>) -> Vec<Point> {
    let mut output = vec![];

    if point.x > 0 {
        output.push(Point::new(point.x - 1, point.y));
    }

    if point.x < input[point.y].len() - 1 {
        output.push(Point::new(point.x + 1, point.y));
    }

    if point.y > 0 {
        output.push(Point::new(point.x, point.y - 1));
    }

    if point.y < input.len() - 1 {
        output.push(Point::new(point.x, point.y + 1));
    }

    output
}

fn available_successors(start: &Point, input: &Vec<String>) -> Vec<Point> {
    let successors = all_successors(start, input);
    let start = input[start.y].chars().nth(start.x).unwrap();
    let start = normalize(start) as usize;

    successors
        .into_iter()
        .filter(|point| {
            let successor = input[point.y].chars().nth(point.x).unwrap();
            let successor = normalize(successor) as usize;

            start >= successor - 1
        })
        .collect::<Vec<Point>>()
}

fn normalize(point: char) -> char {
    match point {
        START => 'a',
        END => 'z',
        _ => point,
    }
}
