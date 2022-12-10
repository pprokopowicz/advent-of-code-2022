#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl Instruction {
    pub fn cycles(&self) -> isize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}
