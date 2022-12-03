pub fn priority(c: &char) -> usize {
    if c.is_lowercase() {
        (*c as usize) - 96
    } else {
        (*c as usize) - 38
    }
}
