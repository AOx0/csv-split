extern crate fs_extra;

use indexed_file::{Indexable, ReadByLine};
use std::path::Path;

pub struct File {
    file: indexed_file::File,
    file_fs: Box<Path>,
}


impl File {
    pub fn lines(&self) -> usize {
        self.file.total_lines()
    }

    pub fn name(&self) -> String {
        String::from(self.file_fs.file_name().unwrap().to_str().unwrap())
    }

    pub fn new(file: &str) -> Option<File> {
        let file_indexed = if let Ok(file) = indexed_file::File::open_raw(file) {
            file
        } else {
            eprintln!("No such file found");
            return None;
        };

        Some(File {
            file: file_indexed,
            file_fs: Box::from(Path::new(file))
        })
    }

    pub fn base_name(&self) -> Option<String> {
        let name = self.name().to_lowercase();

         if name.to_lowercase().contains(".csv") {
            Some(name.replace(".csv", ""))
        } else {
            None
        }
    }
}