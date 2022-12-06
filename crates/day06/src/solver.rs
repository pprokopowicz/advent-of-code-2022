use std::collections::HashSet;

pub fn solver(input: &str, len: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    let mut output = 0;

    for i in 0..=chars.len() - (len + 1) {
        let mut sequence = HashSet::new();

        for j in 0..len {
            sequence.insert(chars[i + j]);
        }

        if sequence.len() == len {
            output = i + len;
            break;
        }
    }

    output
}
