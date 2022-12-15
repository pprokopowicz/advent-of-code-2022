use reader::{read_file, File};

use crate::sensor::{distance, Point, PointPair};

pub fn parse() -> Vec<PointPair> {
    let input = read_file(File::Day15);

    input
        .lines()
        .map(|line| {
            let sensor_x_end = line.find(",").unwrap();
            let sensor_x = &line[12..sensor_x_end];

            let sensor_y_start = sensor_x_end + 4;
            let sensor_y_end = line.find(":").unwrap();
            let sensor_y = &line[sensor_y_start..sensor_y_end];

            let beacon_x_start = sensor_y_end + 25;
            let beacon_x_end = line.rfind(",").unwrap();
            let beacon_x = &line[beacon_x_start..beacon_x_end];

            let beacon_y_start = beacon_x_end + 4;
            let beacon_y_end = line.len();
            let beacon_y = &line[beacon_y_start..beacon_y_end];

            let sensor_x = isize_from(sensor_x);
            let sensor_y = isize_from(sensor_y);
            let beacon_x = isize_from(beacon_x);
            let beacon_y = isize_from(beacon_y);

            let sensor = Point::new(sensor_x, sensor_y);
            let beacon = Point::new(beacon_x, beacon_y);
            let distance = distance(&sensor, &beacon);

            PointPair::new(sensor, beacon, distance)
        })
        .collect::<Vec<PointPair>>()
}

fn isize_from(input: &str) -> isize {
    input.parse::<isize>().unwrap()
}
