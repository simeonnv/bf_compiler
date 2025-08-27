use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::Operation;

lazy_static! {
    pub static ref TokenizationPattern: HashMap<&'static str, Operation> = {
        let mut tokenization_pattern = HashMap::new();

        tokenization_pattern.insert(">", Operation::Right(1));
        tokenization_pattern.insert("<", Operation::Left(1));
        tokenization_pattern.insert("+", Operation::Add(1));
        tokenization_pattern.insert("-", Operation::Sub(1));
        tokenization_pattern.insert("[", Operation::LoopStart);
        tokenization_pattern.insert("]", Operation::LoopEnd);
        tokenization_pattern.insert(".", Operation::Output);
        tokenization_pattern.insert(",", Operation::Input);

        return tokenization_pattern;
    };
}
