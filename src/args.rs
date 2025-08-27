use clap::{Parser, ValueEnum};
use lazy_static::lazy_static;
use std::{path::PathBuf, str::FromStr};

#[derive(ValueEnum, Clone, Debug)]
#[value(rename_all = "lowercase")] // Ensures case-insensitive parsing (optional)
pub enum Mode {
    Int,
    Comp,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long, default_value = "comp")]
    pub mode: Mode,

    #[arg(short, long, default_value_t = 30_000)]
    pub stack_size: usize,

    #[arg(short, long, default_value_t = false)]
    pub time: bool,

    #[arg(short, long, default_value = "./out.o", value_parser = PathBuf::from_str)]
    pub output: PathBuf,
}

lazy_static! {
    pub static ref ARGS: Args = Args::parse();
}
