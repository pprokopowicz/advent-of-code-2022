use reader::read_file;

mod parser;
mod part1;
mod part2;
mod range_ext;

fn main() {
    let input = read_file("crates/day04/input.txt");
    let input = parser::parse(&input);

    part1::solve(&input);
    part2::solve(&input);
}
