mod map;
mod parser;
mod part1;
mod part2;
mod simulate;

fn main() {
    let input = parser::parse();

    part1::solve(&input);
    part2::solve(&input);
}
