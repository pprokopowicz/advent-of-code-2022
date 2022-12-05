pub fn format_out(input: &Vec<Vec<char>>) -> String {
    let mut output = String::new();
    for stack in input {
        output.push(stack[stack.len() - 1]);
    }
    output
}
