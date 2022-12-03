use crate::priority::priority;

pub fn solve(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut index = 0;
    let mut sum = 0;

    while index < lines.len() {
        let line0 = lines[index];
        let line1 = lines[index + 1];
        let line2 = lines[index + 2];

        let c = line0
            .chars()
            .filter(|item| line1.contains(*item) && line2.contains(*item))
            .next()
            .unwrap();

        sum += priority(&c);

        index = index + 3;
    }

    println!("Part 2 solution: {}", sum);
}
