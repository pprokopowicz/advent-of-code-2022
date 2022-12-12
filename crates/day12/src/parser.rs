use reader::{read_file, File};

pub fn parse() -> Vec<String> {
    read_file(File::Day12)
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}
