use std::process::exit;
use crate::args::Args;
use file_manager::File;

pub mod args;
pub mod file_manager;

pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load()
    };

    println!("{:?}", args);
}



pub fn gen_names(file: File, n: i32) -> Vec<String> {

    let base_name = file.base_name();
    println!("{:?}", 3);
    Vec::<String>::new()
}
