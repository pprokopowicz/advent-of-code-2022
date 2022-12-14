use reader::{read_file, File};

pub struct Input {
    pub lhs: String,
    pub rhs: String,
}

pub fn parse() -> Vec<Input> {
    let contents = read_file(File::Day02);

    contents
        .lines()
        .into_iter()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();

            Input {
                lhs: split[0].to_string(),
                rhs: split[1].to_string(),
            }
        })
        .collect::<Vec<Input>>()
}
