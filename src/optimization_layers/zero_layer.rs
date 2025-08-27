use crate::Operation;

// turns zero expressions "[-]" (which are 3 opperations) into a single Zero opperation
pub fn zero_layer(operations: Box<[Operation]>) -> Box<[Operation]> {
    let mut zeroed = Vec::with_capacity(operations.len());

    let mut i = 0;
    while i < operations.len() {
        if i + 2 < operations.len() {
            match (&operations[i], &operations[i + 1], &operations[i + 2]) {
                (Operation::LoopStart, Operation::Sub(_), Operation::LoopEnd) => {
                    zeroed.push(Operation::Zero);
                    i += 3;
                    continue;
                }
                _ => {}
            }
        }

        zeroed.push(operations[i].clone());
        i += 1;
    }

    zeroed.into_boxed_slice()
}
