mod part1;
mod part2;
mod solver;

fn main() {
    let input = reader::read_file("crates/day06/input.txt");

    part1::solve(&input);
    part2::solve(&input);
}
