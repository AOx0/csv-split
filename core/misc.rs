use crate::reader::BufReader;
use crate::*;

pub fn read_n_lines(file: &mut BufReader, next: usize) -> String {
    let mut result = String::new();

    for _ in 1..=next {
        let mut buffer = String::new();
        file.read_line(&mut buffer);
        result.push_str(buffer.as_str());
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

pub fn get_target_directory(file: &File) -> String {
    format!(
        "{}/{}",
        current_dir().unwrap().display(),
        file.base_name().unwrap()
    )
}

pub fn gen_paths(file: &File, n_files: usize) -> Vec<String> {
    let names = misc::gen_names(file, n_files);
    names
        .iter()
        .map(|a| format!("{}/{}", get_target_directory(file), a))
        .collect()
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

#[macro_export]
macro_rules! b_file {
    ($f: expr) => {
        $f.clone().lock().unwrap().deref_mut()
    };
}
