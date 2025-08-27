use std::{path::PathBuf, str::FromStr, time::Instant};

use clap::Parser;

pub mod compiler;
pub mod interpreter;
pub mod optimization_layers;
pub mod tokenizator;

mod operation;
pub use operation::Operation;

mod args;
pub use args::ARGS;

use std::fs;

use crate::{
    compiler::compile::compile,
    interpreter::interpreter_run::interpeter_run,
    optimization_layers::{joiner_layer::joiner_layer, zero_layer::zero_layer},
    tokenizator::tokenize::tokenize,
};

fn main() {
    let bf_file = fs::read(&ARGS.input);
    let bf_file = match bf_file {
        Ok(e) => e,
        Err(e) => panic!("failed to read bf file: {}", e),
    };

    // dbg!(str::from_utf8(&bf_file).unwrap());

    let optimization_time = Instant::now();

    let operations = tokenize(bf_file);
    let operations = joiner_layer(operations);
    let operations = zero_layer(operations);

    let duration = optimization_time.elapsed();
    if ARGS.time {
        println!("optimization time was: {:#?}", duration)
    }

    // interpeter_run(&operations);
    compile(operations)

    // println!("Hello, world!");
}
