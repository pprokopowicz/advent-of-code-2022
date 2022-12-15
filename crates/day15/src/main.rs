mod parser;
mod part1;
mod part2;
mod sensor;

fn main() {
    let input = parser::parse();

    part1::solve(&input);
    part2::solve(&input);
}
