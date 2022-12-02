use crate::opp_choice::opp_choice;
use crate::{
    game::{Choice, GameOutcome},
    parser::Input,
};

pub fn solve(input: &Vec<Input>) {
    let score = input
        .into_iter()
        .map(|input| {
            let opp_choice = opp_choice(&input.lhs);
            let outcome = outcome(&input.rhs);
            let user_choice = user_choice(&opp_choice, &outcome);

            user_choice.user_points() + outcome.user_points()
        })
        .sum::<usize>();

    println!("Part 2 solution {}", score);
}

fn outcome(value: &str) -> GameOutcome {
    match value {
        "X" => GameOutcome::Loss,
        "Y" => GameOutcome::Draw,
        "Z" => GameOutcome::Win,
        _ => panic!("Bad expected outcome"),
    }
}

fn user_choice(opp_choice: &Choice, outcome: &GameOutcome) -> Choice {
    match (opp_choice, outcome) {
        (Choice::Rock, GameOutcome::Loss) => Choice::Scissors,
        (Choice::Rock, GameOutcome::Draw) => Choice::Rock,
        (Choice::Rock, GameOutcome::Win) => Choice::Paper,
        (Choice::Paper, GameOutcome::Loss) => Choice::Rock,
        (Choice::Paper, GameOutcome::Draw) => Choice::Paper,
        (Choice::Paper, GameOutcome::Win) => Choice::Scissors,
        (Choice::Scissors, GameOutcome::Loss) => Choice::Paper,
        (Choice::Scissors, GameOutcome::Draw) => Choice::Scissors,
        (Choice::Scissors, GameOutcome::Win) => Choice::Rock,
    }
}
