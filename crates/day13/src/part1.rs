use crate::packet_order::is_correct_order;
use crate::parser::PacketPair;

pub fn solve(input: &Vec<PacketPair>) {
    let sum = input
        .iter()
        .enumerate()
        .filter_map(
            |(index, pair)| match is_correct_order(&pair.lhs, &pair.rhs) {
                None => None,
                Some(result) => {
                    if result {
                        Some(index + 1)
                    } else {
                        None
                    }
                }
            },
        )
        .sum::<usize>();

    println!("Part 1 solution: {}", sum);
}
