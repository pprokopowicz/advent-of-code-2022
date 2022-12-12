use reader::{read_file, File};

pub fn parse() -> Vec<Vec<char>> {
    read_file(File::Day12)
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
