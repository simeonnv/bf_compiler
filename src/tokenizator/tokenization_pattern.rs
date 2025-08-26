use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::Opperation;

lazy_static! {
    pub static ref TokenizationPattern: HashMap<&'static str, Opperation> = {
        let mut tokenization_pattern = HashMap::new();

        tokenization_pattern.insert(">", Opperation::Right(1));
        tokenization_pattern.insert("<", Opperation::Left(1));
        tokenization_pattern.insert("+", Opperation::Add(1));
        tokenization_pattern.insert("-", Opperation::Sub(1));
        tokenization_pattern.insert("[", Opperation::LoopStart(None));
        tokenization_pattern.insert("]", Opperation::LoopEnd(None));
        tokenization_pattern.insert(".", Opperation::Input);
        tokenization_pattern.insert(",", Opperation::Output);

        return tokenization_pattern;
    };
}
