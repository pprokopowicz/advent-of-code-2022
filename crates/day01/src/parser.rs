use reader::read_file;

pub fn parse() -> Vec<usize> {
    let contents = read_file("crates/day01/input.txt");

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
