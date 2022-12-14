use crate::map::{Map, MapPoint, Point};
use reader::{read_file, File};
use std::cmp::{max, min};

pub fn parse() -> Map {
    let input = read_file(File::Day14);

    let mut x_min = usize::MAX;
    let mut x_max = 0usize;
    let mut y_max = 0usize;

    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let mut p_split = point.split(",");

                    let x = p_split.next().unwrap().parse::<usize>().unwrap();
                    let y = p_split.next().unwrap().parse::<usize>().unwrap();

                    x_min = min(x, x_min);
                    x_max = max(x, x_max);
                    y_max = max(y, y_max);

                    Point::new(x, y)
                })
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let lines = lines
        .iter()
        .map(|points| {
            points
                .iter()
                .map(|point| Point::new(point.x - x_min, point.y))
                .collect::<Vec<Point>>()
        })
        .collect::<Vec<Vec<Point>>>();

    let mut map: Map = vec![vec![MapPoint::Air; (x_max - x_min) + 1]; y_max + 1];
    map[0][500 - x_min] = MapPoint::SandEntrance;

    place_points(&lines, &mut map);

    return map;
}

fn place_points(lines: &Vec<Vec<Point>>, map: &mut Map) {
    lines.iter().for_each(|line| {
        (0..line.len() - 1).for_each(|i| {
            let start = &line[i];
            let end = &line[i + 1];

            if start.x == end.x {
                let y_start = min(start.y, end.y);
                let y_end = max(start.y, end.y);
                (y_start..=y_end).for_each(|y| map[y][start.x] = MapPoint::Rock)
            } else {
                let x_start = min(start.x, end.x);
                let x_end = max(start.x, end.x);
                (x_start..=x_end).for_each(|x| map[start.y][x] = MapPoint::Rock)
            }
        });
    })
}
