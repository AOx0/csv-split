extern crate fs_extra;

use std::ops::Deref;
use indexed_file::{Indexable, ReadByLine};
use std::path::Path;
use std::process::exit;

pub struct File {
    pub file_path: Box<String>,
    file: indexed_file::File,
    file_fs: Box<Path>,
    header_line: bool,
}

impl File {

    pub fn read_line(&mut self, line: usize) -> String {
        self.file.read_line(line).unwrap().to_string()
    }

    pub fn header(&self) -> usize {
        if self.header_line == true {
            1
        } else {
            0
        }
    }

    pub fn lines(&self) -> usize {
        self.file.total_lines()
    }

    pub fn name(&self) -> String {
        String::from(self.file_fs.file_name().unwrap().to_str().unwrap())
    }

    pub fn new(file_p: &str, header_line: bool) -> Option<File> {
        let file = if let Ok(file_p) = indexed_file::File::open_raw(file_p) {
            file_p
        } else {
            eprintln!("No such file found");
            return None;
        };

        let file_path = if std::path::Path::new(&file_p).is_absolute() && std::path::Path::new(&file_p).is_file()  {
            Box::from(file_p.to_string())
        } else {
            eprintln!("Only full path is allowed");
            exit(1);
        };


        Some(File {
            file_path,
            file,
            file_fs: Box::from(Path::new(file_p)),
            header_line,
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

impl Clone for File {
    fn clone(&self) -> Self {
        File::new(self.file_path.deref(), self.header_line).unwrap()
    }
}