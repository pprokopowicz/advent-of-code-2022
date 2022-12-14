use crate::map::{sand_entrance, Map, MapPoint};
use crate::simulate::simulate_sand;

pub fn solve(map: &Map) {
    let mut map = map.clone();
    let entrance = sand_entrance(&map);

    let mut counter = 0;
    while let Some(point) = simulate_sand(&entrance, &mut map) {
        map[point.y][point.x] = MapPoint::Sand;
        counter += 1;
    }

    println!("Part 1 solution: {}", counter);
}
