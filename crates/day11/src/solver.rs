use crate::monkey::{Monkey, Operation};

pub fn solve(input: &mut Vec<Monkey>, rounds: usize, divide: bool) -> usize {
    let sm = input
        .iter()
        .map(|m| m.test_number)
        .reduce(|acc, element| acc * element)
        .unwrap();

    (0..rounds).for_each(|_| {
        (0..input.len()).for_each(|index| {
            let mut clone = input[index].clone();

            (0..clone.items.len()).for_each(|_| inspect_item(&mut clone, input, divide, sm));

            input[index] = clone;
        });
    });

    let mut inspected_vec = input
        .iter()
        .map(|monkey| monkey.num_inspected)
        .collect::<Vec<usize>>();
    inspected_vec.sort();

    inspected_vec[inspected_vec.len() - 1] * inspected_vec[inspected_vec.len() - 2]
}

fn inspect_item(monkey: &mut Monkey, monkey_vec: &mut Vec<Monkey>, divide: bool, sm: usize) {
    let item = monkey.items.remove(0) % sm;

    let mut new_item = match monkey.operation {
        Operation::Add(num) => item + num,
        Operation::Multiply(num) => item * num,
        Operation::Square => item * item,
    };

    if divide {
        new_item /= 3;
    }

    if new_item % monkey.test_number == 0 {
        monkey_vec[monkey.test_success].items.push(new_item);
    } else {
        monkey_vec[monkey.test_failure].items.push(new_item);
    }

    monkey.num_inspected += 1;
}
