use reader::read_file;

#[derive(Clone, Debug)]
pub struct Instruction {
    pub amount: usize,
    pub origin: usize,
    pub destination: usize,
}

#[derive(Clone, Debug)]
pub struct Input {
    pub stacks: Vec<Vec<char>>,
    pub instructions: Vec<Instruction>,
}

pub fn parse() -> Input {
    let input = read_file("crates/day05/input.txt");
    let split = input.split("\n\n").collect::<Vec<&str>>();

    Input {
        stacks: parse_stack(split[0]),
        instructions: parse_instructions(split[1]),
    }
}

fn parse_stack(input: &str) -> Vec<Vec<char>> {
    let mut stacks = input
        .lines()
        .map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|chars| chars[1])
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    stacks.pop();
    stacks.reverse();
    let mut flipped_stacks: Vec<Vec<char>> = vec![];
    for _ in 0..stacks[0].len() {
        flipped_stacks.push(Vec::new());
    }

    for stack in stacks {
        for (index, c) in stack.iter().enumerate() {
            if !c.is_whitespace() {
                flipped_stacks[index].push(c.clone());
            }
        }
    }

    flipped_stacks
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();

            let amount = split[1].parse::<usize>().unwrap();
            let origin = split[3].parse::<usize>().unwrap();
            let destination = split[5].parse::<usize>().unwrap();

            Instruction {
                amount,
                origin,
                destination,
            }
        })
        .collect::<Vec<Instruction>>()
}
