use std::cmp;

use crate::packet_order::is_correct_order;
use crate::packet_type::PacketType;
use crate::parser::PacketPair;

pub fn solve(input: &Vec<PacketPair>) {
    let div_packet_1 = vec![PacketType::List(vec![PacketType::Number(2)])];
    let div_packet_2 = vec![PacketType::List(vec![PacketType::Number(6)])];

    let cloned = input.clone();
    let mut input = cloned
        .into_iter()
        .flat_map(|pair| vec![&pair.lhs, &pair.rhs])
        .collect::<Vec<&Vec<PacketType>>>();
    input.push(&div_packet_1);
    input.push(&div_packet_2);

    input.sort_by(|lhs, rhs| match is_correct_order(lhs, rhs) {
        None => cmp::Ordering::Equal,
        Some(result) => {
            if result {
                cmp::Ordering::Less
            } else {
                cmp::Ordering::Greater
            }
        }
    });

    let indices = input
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if *packet == &div_packet_1 || *packet == &div_packet_2 {
                return Some(index + 1);
            }

            return None;
        })
        .collect::<Vec<usize>>();

    println!("Part 2 solution: {}", indices[0] * indices[1]);
}
