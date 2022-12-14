use crate::map::{Map, MapPoint, Point};

pub fn simulate_sand(sand_entrance: &Point, map: &mut Map) -> Option<Point> {
    let mut point = Some(Point::new(sand_entrance.x, sand_entrance.y));

    while let Some(new_point) = move_once(point.as_ref(), map) {
        point = Some(new_point);
    }

    if let Some(point) = point {
        if map[sand_entrance.y][sand_entrance.x] != MapPoint::Sand
            && point.x != 0
            && point.x != map[point.y].len() - 1
            && point.y != map.len() - 1
        {
            return Some(point);
        }
    }

    return None;
}

fn move_once(sand: Option<&Point>, map: &Map) -> Option<Point> {
    let sand = sand?;

    let new_y = sand.y + 1;
    if new_y < map.len() && map[new_y][sand.x] == MapPoint::Air {
        return Some(Point::new(sand.x, new_y));
    }

    let left_x = (sand.x as isize) - 1;
    if new_y < map.len() && left_x >= 0 && map[new_y][left_x as usize] == MapPoint::Air {
        return Some(Point::new(left_x as usize, new_y));
    }

    let right_x = sand.x + 1;
    if new_y < map.len() && right_x < map[new_y].len() && map[new_y][right_x] == MapPoint::Air {
        return Some(Point::new(right_x, new_y));
    }

    None
}
