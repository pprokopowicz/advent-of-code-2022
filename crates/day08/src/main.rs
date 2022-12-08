mod parser;
mod part1;
mod part2;

fn main() {
    let input = parser::parse();

    part1::solve(&input);
    part2::solve(&input);
}
