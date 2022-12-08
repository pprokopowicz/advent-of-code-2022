pub fn solve(input: &Vec<Vec<u32>>) {
    let mut output = 0;

    for y in 1..(input.len() - 1) {
        for x in 1..(input[y].len() - 1) {
            if is_visible_top(x, y, input)
                || is_visible_bottom(x, y, input)
                || is_visible_left(x, y, input)
                || is_visible_right(x, y, input)
            {
                output += 1;
            }
        }
    }

    let edges = (input.len() * 2) + (input[0].len() * 2) - 4;
    output += edges;

    println!("Part 1 solution: {}", output);
}

fn is_visible_top(x: usize, y: usize, input: &Vec<Vec<u32>>) -> bool {
    let value = input[y][x];

    (0..y).into_iter().all(|y| input[y][x] < value)
}

fn is_visible_bottom(x: usize, y: usize, input: &Vec<Vec<u32>>) -> bool {
    let value = input[y][x];

    ((y + 1)..input.len())
        .into_iter()
        .all(|y| input[y][x] < value)
}

fn is_visible_left(x: usize, y: usize, input: &Vec<Vec<u32>>) -> bool {
    let value = input[y][x];

    (0..x).into_iter().all(|x| input[y][x] < value)
}

fn is_visible_right(x: usize, y: usize, input: &Vec<Vec<u32>>) -> bool {
    let value = input[y][x];

    ((x + 1)..input[y].len())
        .into_iter()
        .all(|x| input[y][x] < value)
}
