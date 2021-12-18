use std::fs::OpenOptions;
use std::io::Write;
use std::ops::Range;
use crate::*;

pub fn read_line_range(mut file: File, range: Range<usize>) -> Vec<String> {
    let mut result = Vec::<String>::new();
    for i in range {
        result.push(file.read_line(i));
    }

    result
}

pub fn lines_per_file(file: &File, n_files: usize) -> Option<(usize, usize)> {
    let total_lines = file.lines() - file.header();
    let lines = total_lines as f32 / n_files as f32;

    if lines < 1.0 {
        None /* Cant place less than a row in each file */
    } else {
        let lines = lines.floor() as usize;
        Some((lines, total_lines - lines * n_files))
    }
}

pub fn gen_names(file: &File, n_files: usize) -> Vec<String> {
    let mut result = Vec::<String>::new();
    let base_name = file.base_name().unwrap_or_else(|| {
        exit(0);
    });

    for n in 1..=n_files {
        result.push(format!("{}_{}.csv", base_name, n));
    }

    result
}

pub fn write_contents_to(path: &str, contents: &[u8]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;
    file.write(contents)?;
    Ok(())
}