use std::ops::Range;
use crate::args::Args;

pub mod args;
pub mod file_manager;

pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load()
    };

    println!("{:?}", args);
}

