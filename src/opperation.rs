#[derive(Debug, Clone, PartialEq)]
pub enum Opperation {
    Right(usize),
    Left(usize),
    Add(usize),
    Sub(usize),
    LoopStart(Option<usize>),
    LoopEnd(Option<usize>),
    Output,
    Input,
}
