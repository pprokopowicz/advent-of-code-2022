#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

pub fn distance(lhs: &Point, rhs: &Point) -> usize {
    let distance = (lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs();
    usize::try_from(distance).unwrap()
}

#[derive(Debug)]
pub struct PointPair {
    pub sensor: Point,
    pub beacon: Point,
    pub distance: usize,
}

impl PointPair {
    pub fn new(sensor: Point, beacon: Point, distance: usize) -> Self {
        Self {
            sensor,
            beacon,
            distance,
        }
    }
}

pub fn is_point_available(point: &Point, pairs: &Vec<PointPair>) -> bool {
    let distances = pairs
        .into_iter()
        .map(|sensor| (sensor.distance, distance(&sensor.sensor, &point)))
        .collect::<Vec<(usize, usize)>>();

    distances.iter().all(|distance| distance.0 < distance.1)
}
