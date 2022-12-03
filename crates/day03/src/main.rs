use reader::read_file;

mod part1;
mod part2;
mod priority;

fn main() {
    let input = read_file("crates/day03/input.txt");

    part1::solve(&input);
    part2::solve(&input);
}
