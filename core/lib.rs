use crate::args::Args;
use file_manager::File;
use std::process::exit;

pub mod args;
pub mod file_manager;

pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load(),
    };

    println!("{:?}", args);
}

pub fn lines_per_file(file: &File, n_files: usize) -> Option<(usize, usize)> {
    let total_lines = file.lines() - file.header();
    let lines = total_lines as f32 / n_files as f32;

    if lines < 1.0 {
        None /* Cant place less than row in each file */
    } else {
        let lines = lines.floor() as usize;
        Some((lines, total_lines - lines * n_files))
    }
}

pub fn gen_names(file: &File, n_files: i32) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let base_name = file.base_name().unwrap_or_else(|| {
        exit(0);
    });

    for n in 1..=n_files {
        result.push(format!("{}_{}.csv", base_name, n));
    }

    result
}
