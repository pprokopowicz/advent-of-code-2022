use crate::priority::priority;

pub fn solve(input: &str) {
    let priority = input
        .lines()
        .map(|line| {
            let (compartmen0, compartment1) = line.split_at(line.len() / 2);

            let c = compartmen0
                .chars()
                .filter(|item| compartment1.contains(*item))
                .next()
                .unwrap();

            priority(&c)
        })
        .sum::<usize>();

    println!("Part 1 solution: {}", priority);
}
