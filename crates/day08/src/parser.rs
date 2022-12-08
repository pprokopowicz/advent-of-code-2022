use reader::{read_file, File};

pub fn parse() -> Vec<Vec<u32>> {
    let input = read_file(File::Day08);

    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
