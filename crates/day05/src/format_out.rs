pub fn format_out(input: &Vec<Vec<char>>) -> String {
    input
        .iter()
        .map(|stack| stack[stack.len() - 1])
        .collect::<String>()
}
