use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn shift(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y = self.y + 1,
            Direction::Down => self.y = self.y - 1,
            Direction::Left => self.x = self.x - 1,
            Direction::Right => self.x = self.x + 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}
