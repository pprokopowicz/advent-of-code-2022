use crate::game::Choice;

pub fn opp_choice(choice: &str) -> Choice {
    match choice {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Bad opponnent choice"),
    }
}
