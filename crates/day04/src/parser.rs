use std::ops::RangeInclusive;

pub struct Input {
    pub lhs: RangeInclusive<usize>,
    pub rhs: RangeInclusive<usize>,
}

pub fn parse(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|line| {
            let ranges = line.split(',').collect::<Vec<&str>>();
            let range0 = ranges[0]
                .split('-')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let range1 = ranges[1]
                .split('-')
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            Input {
                lhs: range0[0]..=range0[1],
                rhs: range1[0]..=range1[1],
            }
        })
        .collect::<Vec<Input>>()
}
