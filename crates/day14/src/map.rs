#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapPoint {
    Sand,
    Rock,
    Air,
    SandEntrance,
}

pub type Map = Vec<Vec<MapPoint>>;

pub fn sand_entrance(map: &Map) -> Point {
    let x_entrance = map[0]
        .iter()
        .position(|point| *point == MapPoint::SandEntrance)
        .unwrap();
    Point::new(x_entrance, 0)
}
