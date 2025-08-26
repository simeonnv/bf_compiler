use std::{path::PathBuf, str::FromStr};

use clap::Parser;

pub mod interpreter;
pub mod optimization_layers;
pub mod tokenizator;

mod opperation;
pub use opperation::Opperation;

mod args;
pub use args::ARGS;

use std::fs;

use crate::{optimization_layers::joiner_layer::joiner_layer, tokenizator::tokenize::tokenize};

fn main() {
    let bf_file = fs::read(&ARGS.input);
    let bf_file = match bf_file {
        Ok(e) => e,
        Err(e) => panic!("failed to read bf file: {}", e),
    };

    dbg!(str::from_utf8(&bf_file).unwrap());

    let opperations = tokenize(bf_file);
    let opperations = joiner_layer(&opperations);

    dbg!(opperations);

    println!("Hello, world!");
}
