use parser::parse;

mod game;
mod opp_choice;
mod parser;
mod part1;
mod part2;

fn main() {
    let input = parse();

    part1::solve(&input);
    part2::solve(&input);
}
