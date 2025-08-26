use std::ops::Deref;

use aho_corasick::{AhoCorasick, MatchKind, PatternID};

use crate::{Opperation, opperation, tokenizator::tokenization_pattern::TokenizationPattern};

// converts a bf file buff into a simple instructions of non optimised operations
pub fn tokenize(bf_raw_buff: Vec<u8>) -> Vec<Opperation> {
    let bf_string_buff = match String::from_utf8(bf_raw_buff) {
        Ok(string) => string,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    dbg!(&bf_string_buff);

    let aho_corasick_pattern = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .match_kind(MatchKind::LeftmostLongest)
        .build(TokenizationPattern.keys())
        .unwrap();

    let mut operations = Vec::<Opperation>::with_capacity(32);

    for token in aho_corasick_pattern.find_iter(&bf_string_buff) {
        let token = &bf_string_buff[token.start()..token.end()];
        dbg!(token);
        let opperation = TokenizationPattern
            .get(token)
            .expect("failed tokenization, extracted a non-existant token")
            .clone();
        operations.push(opperation);
    }
    dbg!(&operations);

    operations
}
