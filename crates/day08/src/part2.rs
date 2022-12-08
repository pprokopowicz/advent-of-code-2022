pub fn solve(input: &Vec<Vec<u32>>) {
    let mut output = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let score = score_top(x, y, input)
                * score_bottom(x, y, input)
                * score_left(x, y, input)
                * score_right(x, y, input);
            if score > output {
                output = score;
            }
        }
    }

    println!("Part 2 solution: {}", output);
}

fn score_top(x: usize, y: usize, input: &Vec<Vec<u32>>) -> usize {
    let value = input[y][x];
    let mut output = 0;

    for y in (0..y).rev() {
        output += 1;
        if input[y][x] >= value {
            break;
        }
    }

    output
}

fn score_bottom(x: usize, y: usize, input: &Vec<Vec<u32>>) -> usize {
    let value = input[y][x];
    let mut output = 0;

    for y in (y + 1)..input.len() {
        output += 1;
        if input[y][x] >= value {
            break;
        }
    }

    output
}

fn score_left(x: usize, y: usize, input: &Vec<Vec<u32>>) -> usize {
    let value = input[y][x];
    let mut output = 0;

    for x in (0..x).rev() {
        output += 1;
        if input[y][x] >= value {
            break;
        }
    }

    output
}

fn score_right(x: usize, y: usize, input: &Vec<Vec<u32>>) -> usize {
    let value = input[y][x];
    let mut output = 0;

    for x in (x + 1)..input[y].len() {
        output += 1;
        if input[y][x] >= value {
            break;
        }
    }

    output
}
