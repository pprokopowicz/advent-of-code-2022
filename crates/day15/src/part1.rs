use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashSet;

use crate::sensor::{is_point_available, Point, PointPair};

pub fn solve(input: &Vec<PointPair>) {
    let sensors = sensors(input);
    let beacons = beacons(input);

    let (x_min, x_max) = min_max(&input);

    let output = (x_min..=x_max)
        .into_par_iter()
        .filter(|x| {
            let point = Point::new(*x, 2000000);
            if sensors.contains(&point) || beacons.contains(&point) {
                return false;
            }

            !is_point_available(&point, input)
        })
        .count();

    println!("Part 1 solution: {}", output);
}

fn min_max(input: &Vec<PointPair>) -> (isize, isize) {
    let min_max = input.iter().map(|sensor| {
        let distance = isize::try_from(sensor.distance).unwrap();
        (
            min(sensor.sensor.x - distance, sensor.beacon.x - distance),
            max(sensor.sensor.x + distance, sensor.beacon.x + distance),
        )
    });

    (
        min_max.clone().map(|min_max| min_max.0).min().unwrap(),
        min_max.map(|min_max| min_max.1).max().unwrap(),
    )
}


fn sensors(input: &Vec<PointPair>) -> HashSet<&Point> {
    input
        .iter()
        .map(|sensor| &sensor.sensor)
        .collect::<HashSet<&Point>>()
}

fn beacons(input: &Vec<PointPair>) -> HashSet<&Point> {
    input
        .iter()
        .map(|sensor| &sensor.beacon)
        .collect::<HashSet<&Point>>()
}