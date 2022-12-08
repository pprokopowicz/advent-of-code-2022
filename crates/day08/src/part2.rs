pub fn solve(input: &Vec<Vec<u32>>) {
    let mut output = 0;

    input.iter().enumerate().for_each(|(y, x_vec)| {
        x_vec.iter().enumerate().for_each(|(x, value)| {
            let score = score_top(x, y, input, value)
                * score_bottom(x, y, input, value)
                * score_left(x, y, input, value)
                * score_right(x, y, input, value);
            if score > output {
                output = score;
            }
        })
    });

    println!("Part 2 solution: {}", output);
}

fn score_top(x: usize, y: usize, input: &Vec<Vec<u32>>, value: &u32) -> usize {
    if y == 0 {
        return 0;
    }
    let mut output = 0;

    for y in (0..y).rev() {
        if &input[y][x] >= value {
            output += 1;
            break;
        }
        output += 1;
    }

    output
}

fn score_bottom(x: usize, y: usize, input: &Vec<Vec<u32>>, value: &u32) -> usize {
    if y == input.len() - 1 {
        return 0;
    }
    let mut output = 0;

    for y in (y + 1)..input.len() {
        if &input[y][x] >= value {
            output += 1;
            break;
        }
        output += 1;
    }

    output
}

fn score_left(x: usize, y: usize, input: &Vec<Vec<u32>>, value: &u32) -> usize {
    if x == 0 {
        return 0;
    }
    let mut output = 0;

    for x in (0..x).rev() {
        if &input[y][x] >= value {
            output += 1;
            break;
        }
        output += 1;
    }

    output
}

fn score_right(x: usize, y: usize, input: &Vec<Vec<u32>>, value: &u32) -> usize {
    if x == input[y].len() - 1 {
        return 0;
    }
    let mut output = 0;

    for x in (x + 1)..input[y].len() {
        if &input[y][x] >= value {
            output += 1;
            break;
        }
        output += 1;
    }

    output
}
