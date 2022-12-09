mod parser;
mod part1;
mod part2;
mod point;
mod shift;

fn main() {
    let input = parser::parse();

    part1::solve(&input);
    part2::solve(&input);
}
