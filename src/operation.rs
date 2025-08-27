#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    Right(usize),
    Left(usize),
    Add(u8),
    Sub(u8),
    LoopStart,
    LoopEnd,
    Zero,
    Output,
    Input,
}
