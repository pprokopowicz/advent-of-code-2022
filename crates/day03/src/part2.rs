use crate::priority::priority;

pub fn solve(input: &str) {
    let lines = input.lines().collect::<Vec<&str>>();

    let sum = lines
        .chunks(3)
        .map(|chunk| {
            let c = chunk[0]
                .chars()
                .filter(|item| chunk[1].contains(*item) && chunk[2].contains(*item))
                .next()
                .unwrap();

            priority(&c)
        })
        .sum::<usize>();

    println!("Part 2 solution: {}", sum);
}
