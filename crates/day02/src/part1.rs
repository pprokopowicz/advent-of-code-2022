use crate::game::{Choice, GameOutcome};
use crate::opp_choice::opp_choice;
use crate::parser::parse;

pub fn solve() {
    let input = parse();

    let score = input
        .into_iter()
        .map(|input| {
            let opp_choice = opp_choice(&input.lhs);
            let user_choice = user_choice(&input.rhs);
            let user_outcome = user_outcome(&user_choice, &opp_choice);

            user_choice.user_points() + user_outcome.user_points()
        })
        .sum::<usize>();

    println!("Part 1 solution {}", score);
}

fn user_choice(choice: &str) -> Choice {
    match choice {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Bad user choice"),
    }
}

fn user_outcome(user_choice: &Choice, opp_choice: &Choice) -> GameOutcome {
    match (user_choice, opp_choice) {
        (Choice::Rock, Choice::Rock) => GameOutcome::Draw,
        (Choice::Rock, Choice::Paper) => GameOutcome::Loss,
        (Choice::Rock, Choice::Scissors) => GameOutcome::Win,
        (Choice::Paper, Choice::Rock) => GameOutcome::Win,
        (Choice::Paper, Choice::Paper) => GameOutcome::Draw,
        (Choice::Paper, Choice::Scissors) => GameOutcome::Loss,
        (Choice::Scissors, Choice::Rock) => GameOutcome::Loss,
        (Choice::Scissors, Choice::Paper) => GameOutcome::Win,
        (Choice::Scissors, Choice::Scissors) => GameOutcome::Draw,
    }
}
