use crate::Opperation;

// joins sequences of opperations together into single opperations
// should be the first layer right after tokenization
pub fn joiner_layer(operations: &[Opperation]) -> Vec<Opperation> {
    let mut joined: Vec<Opperation> = Vec::with_capacity(operations.len());
    let mut i = 0;

    while i < operations.len() {
        match &operations[i] {
            Opperation::Right(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Opperation::Right(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Opperation::Right(total));
            }

            Opperation::Left(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Opperation::Left(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Opperation::Left(total));
            }

            Opperation::Add(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Opperation::Add(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Opperation::Add(total));
            }

            Opperation::Sub(n) => {
                let mut total = *n;
                i += 1;
                while i < operations.len() {
                    if let Opperation::Sub(m) = &operations[i] {
                        total = total.saturating_add(*m);
                        i += 1;
                    } else {
                        break;
                    }
                }
                joined.push(Opperation::Sub(total));
            }

            Opperation::Output => {
                joined.push(Opperation::Output);
                i += 1;
            }
            Opperation::Input => {
                joined.push(Opperation::Input);
                i += 1;
            }
            Opperation::LoopStart(pos) => {
                joined.push(Opperation::LoopStart(*pos));
                i += 1;
            }
            Opperation::LoopEnd(pos) => {
                joined.push(Opperation::LoopEnd(*pos));
                i += 1;
            }
        }
    }

    joined
}
