use reader::{read_file, File};

mod part1;
mod part2;
mod solver;

fn main() {
    let input = read_file(File::Day06);

    part1::solve(&input);
    part2::solve(&input);
}
