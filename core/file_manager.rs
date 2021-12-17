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

    pub fn new(file: &str) -> File {
        File {
            file: indexed_file::File::open_raw(file).unwrap(),
            file_fs: Box::from(Path::new(file))
        }
    }
}