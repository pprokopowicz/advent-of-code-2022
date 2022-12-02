#[derive(Debug)]
pub enum GameOutcome {
    Win,
    Loss,
    Draw,
}

impl GameOutcome {
    pub fn user_points(&self) -> usize {
        match self {
            GameOutcome::Loss => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Win => 6,
        }
    }
}

#[derive(Debug)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn user_points(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}
