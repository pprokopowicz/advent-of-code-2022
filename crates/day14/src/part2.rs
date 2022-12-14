use crate::map::{sand_entrance, Map, MapPoint};
use crate::simulate::simulate_sand;

pub fn solve(map: &Map) {
    let mut map = enlarge_map(&mut map.clone());
    let entrance = sand_entrance(&map);

    let mut counter = 0;
    while let Some(point) = simulate_sand(&entrance, &mut map) {
        map[point.y][point.x] = MapPoint::Sand;
        counter += 1;
    }

    println!("Part 2 solution: {}", counter);
}

fn enlarge_map(map: &mut Map) -> Map {
    let space = calculate_missing_space(&map);
    let mut map = map
        .iter_mut()
        .map(|row| {
            let mut output = vec![MapPoint::Air; space.0];

            let mut right_space = vec![MapPoint::Air; space.1];

            output.append(row);
            output.append(&mut right_space);

            output
        })
        .collect::<Map>();

    map.push(vec![MapPoint::Air; map[0].len()]);
    map.push(vec![MapPoint::Rock; map[0].len()]);

    map
}

fn calculate_missing_space(map: &Map) -> (usize, usize) {
    let entrance = sand_entrance(&map);
    let left = map.len() / 2 + 1;
    let right = (entrance.x + left) - (map[0].len() - entrance.x) + 1;

    (left, right)
}
