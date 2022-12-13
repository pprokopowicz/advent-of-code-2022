use crate::packet_type::PacketType;

pub fn is_correct_order(lhs: &Vec<PacketType>, rhs: &Vec<PacketType>) -> Option<bool> {
    for it in lhs.iter().zip(rhs.iter()) {
        let (lhs, rhs) = it;

        match (lhs, rhs) {
            (PacketType::Number(lhs_num), PacketType::Number(rhs_num)) => {
                if lhs_num > rhs_num {
                    return Some(false);
                } else if lhs_num < rhs_num {
                    return Some(true);
                }
            }
            (PacketType::Number(lhs_num), PacketType::List(rhs_vec)) => {
                match is_correct_order(&vec![PacketType::Number(*lhs_num)], rhs_vec) {
                    None => {}
                    Some(result) => return Some(result),
                }
            }
            (PacketType::List(lhs_vec), PacketType::Number(rhs_num)) => {
                match is_correct_order(lhs_vec, &vec![PacketType::Number(*rhs_num)]) {
                    None => {}
                    Some(result) => return Some(result),
                }
            }
            (PacketType::List(lhs_vec), PacketType::List(rhs_vec)) => {
                match is_correct_order(lhs_vec, rhs_vec) {
                    None => {}
                    Some(result) => return Some(result),
                }
            }
        }
    }

    if lhs.len() == rhs.len() {
        return None;
    }

    return Some(lhs.len() < rhs.len());
}
