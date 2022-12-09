mod parser;
mod part1;
mod part2;

fn main() {
    let mut values = parser::parse();

    part1::solve(&values);
    part2::solve(&mut values);
}
