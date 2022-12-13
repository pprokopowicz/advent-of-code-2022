use reader::{read_file, File};

use crate::packet_type::PacketType;

pub fn parse() -> Vec<PacketPair> {
    let input = read_file(File::Day13);
    input
        .split("\n\n")
        .map(|packets| {
            let mut split = packets.lines();
            let lhs = parse_packet_type(split.next().unwrap());
            let rhs = parse_packet_type(split.next().unwrap());

            PacketPair { lhs, rhs }
        })
        .collect::<Vec<PacketPair>>()
}

fn parse_packet_type(input: &str) -> Vec<PacketType> {
    serde_json::from_str(input).unwrap()
}

#[derive(Debug)]
pub struct PacketPair {
    pub lhs: Vec<PacketType>,
    pub rhs: Vec<PacketType>,
}
