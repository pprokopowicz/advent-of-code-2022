use reader::{read_file, File};

mod part1;
mod part2;
mod priority;

fn main() {
    let input = read_file(File::Day03);

    part1::solve(&input);
    part2::solve(&input);
}
