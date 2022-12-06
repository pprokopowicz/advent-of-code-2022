use reader::{read_file, File};

pub fn parse() -> Vec<usize> {
    let contents = read_file(File::Day01);

    let split = contents.split("\n\n").collect::<Vec<&str>>();

    split
        .into_iter()
        .map(|val| {
            val.lines()
                .map(|line| line.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>()
}
