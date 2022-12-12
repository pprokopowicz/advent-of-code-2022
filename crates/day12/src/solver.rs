use pathfinding::prelude::bfs;

use crate::point::Point;

pub const START: char = 'S';
const END: char = 'E';

pub fn solve(starting_char: Vec<char>, input: &Vec<String>) -> usize {
    let start = starting_char
        .iter()
        .flat_map(|c| find(*c, input))
        .collect::<Vec<Point>>();

    let end = &find(END, input)[0];
    let mut shortest_len = usize::MAX;

    start.iter().for_each(|start| {
        let result = bfs(
            start,
            |point| available_successors(&point, input),
            |point| point == end,
        );

        match result {
            Some(result) => {
                if shortest_len > result.len() {
                    shortest_len = result.len()
                }
            }
            None => {}
        };
    });

    shortest_len - 1
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

    let can_move_left = point.x > 0;
    let can_move_right = point.x < input[point.y].len() - 1;
    let can_move_up = point.y > 0;
    let can_move_down = point.y < input.len() - 1;

    if can_move_left {
        output.push(Point::new(point.x - 1, point.y));
    }

    if can_move_right {
        output.push(Point::new(point.x + 1, point.y));
    }

    if can_move_up {
        output.push(Point::new(point.x, point.y - 1));
    }

    if can_move_down {
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
