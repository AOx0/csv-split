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

pub fn calc_files_to_create(file: &File, n: i32) -> usize {
    (file.lines() as f32 / n as f32).ceil() as usize
}

pub fn gen_names(file: &File, n: i32) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let base_name = file.base_name().unwrap_or_else(|| {
        exit(0);
    });

    for n in 1..=n {
        result.push(format!("{}_{}.csv", base_name, n));
    }

    result
}
