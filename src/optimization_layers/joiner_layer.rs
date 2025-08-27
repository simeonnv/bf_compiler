use crate::Operation;

// joins sequences of opperations together into single opperations
pub fn joiner_layer(operations: Box<[Operation]>) -> Box<[Operation]> {
    let mut joined: Vec<Operation> = Vec::with_capacity(operations.len());
    let mut i = 0;

    while i < operations.len() {
        match &operations[i] {
            Operation::Right(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Operation::Right(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Operation::Right(total));
            }

            Operation::Left(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Operation::Left(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Operation::Left(total));
            }

            Operation::Add(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Operation::Add(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Operation::Add(total));
            }

            Operation::Sub(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Operation::Sub(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Operation::Sub(total));
            }

            Operation::Output => {
                joined.push(Operation::Output);
                i += 1;
            }
            Operation::Input => {
                joined.push(Operation::Input);
                i += 1;
            }
            Operation::LoopStart => {
                joined.push(Operation::LoopStart);
                i += 1;
            }
            Operation::LoopEnd => {
                joined.push(Operation::LoopEnd);
                i += 1;
            }
            Operation::Zero => {
                joined.push(Operation::Zero);
                i += 1;
            }
        }
    }

    joined.into_boxed_slice()
}
