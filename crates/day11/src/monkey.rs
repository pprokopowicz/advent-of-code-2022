#[derive(Clone, Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    pub operation: Operation,
    pub test_number: usize,
    pub test_success: usize,
    pub test_failure: usize,
    pub num_inspected: usize,
}

#[derive(Clone, Debug)]
pub enum Operation {
    Multiply(usize),
    Add(usize),
    Square,
}
