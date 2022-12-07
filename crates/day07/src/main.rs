mod parser;
mod part1;
mod part2;
mod sizes;

fn main() {
    let input = parser::parse();

    part1::solve(&input.borrow());
    part2::solve(&input.borrow());
}
