use pathfinding::prelude::bfs;
use std::cmp::min;

use crate::point::Point;

pub const START: char = 'S';
const END: char = 'E';

pub fn solve(starting_char: Vec<char>, input: &Vec<Vec<char>>) -> usize {
    let end = &find(END, input)[0];

    starting_char
        .iter()
        .flat_map(|c| find(*c, input))
        .filter_map(|point| {
            Some(
                bfs(
                    &point,
                    |point| successors(&point, input),
                    |point| point == end,
                )?
                .len(),
            )
        })
        .reduce(|shortest, path| min(shortest, path))
        .expect("No path found")
        - 1
}

fn find(c: char, input: &Vec<Vec<char>>) -> Vec<Point> {
    input
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            let index = line.iter().position(|element| element == &c);

            match index {
                None => None,
                Some(x) => Some(Point::new(x, y)),
            }
        })
        .collect()
}

fn successors(point: &Point, input: &Vec<Vec<char>>) -> Vec<Point> {
    let start = normalize(input[point.y][point.x]) as usize;
    let mut output = vec![];

    if point.x > 0 {
        let successor = normalize(input[point.y][point.x - 1]) as usize - 1;
        if start >= successor {
            output.push(Point::new(point.x - 1, point.y));
        }
    }

    if point.x < input[point.y].len() - 1 {
        let successor = normalize(input[point.y][point.x + 1]) as usize - 1;
        if start >= successor {
            output.push(Point::new(point.x + 1, point.y));
        }
    }

    if point.y > 0 {
        let successor = normalize(input[point.y - 1][point.x]) as usize - 1;
        if start >= successor {
            output.push(Point::new(point.x, point.y - 1));
        }
    }

    if point.y < input.len() - 1 {
        let successor = normalize(input[point.y + 1][point.x]) as usize - 1;
        if start >= successor {
            output.push(Point::new(point.x, point.y + 1));
        }
    }

    output
}

fn normalize(point: char) -> char {
    match point {
        START => 'a',
        END => 'z',
        _ => point,
    }
}
