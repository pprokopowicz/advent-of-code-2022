use std::collections::HashSet;

use rayon::prelude::*;

use crate::sensor::{is_point_available, Point, PointPair};

const MAX: isize = 4000000;
const MULTIPLIER: isize = 4000000;

pub fn solve(input: &Vec<PointPair>) {
    let point = input
        .into_par_iter()
        .find_map_first(|pair| {
            let perimiter = isize::try_from(pair.distance + 1).expect("TEST ERROR MSG");
            let sensor = &pair.sensor;

            (0..=perimiter).find_map(|i| {
                let points = possible_points(
                    sensor.x + perimiter - i, 
                    sensor.x + i - perimiter, 
                    sensor.y + i, 
                    sensor.y - i
                );

                points
                    .into_iter()
                    .find(|point| is_point_available(point, input))
            })
        })
        .expect("No point available");

    let output = point.x * MULTIPLIER + point.y;
    println!("Part 2 solution: {}", output);
}

fn possible_points(x1: isize, x2: isize, y1: isize, y2: isize) -> HashSet<Point> {
    let is_x1_available = x1 >= 0 && x1 <= MAX;
    let is_x2_available = x2 >= 0 && x2 <= MAX;
    let is_y1_available = y1 >= 0 && y1 <= MAX;
    let is_y2_available = y2 >= 0 && y2 <= MAX;

    let mut output = HashSet::new();

    if is_x1_available && is_y1_available {
        output.insert(Point::new(x1, y1));
    }

    if is_x1_available && is_y2_available {
        output.insert(Point::new(x1, y2));
    }

    if is_x2_available && is_y1_available {
        output.insert(Point::new(x2, y1));
    }

    if is_x2_available && is_y2_available {
        output.insert(Point::new(x2, y2));
    }

    output
}
