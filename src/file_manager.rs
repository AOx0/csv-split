use indexed_file::{Indexable, ReadByLine};

pub struct File {
    file: indexed_file::File
}


impl File {
    pub fn lines(self) -> usize {
        self.file.total_lines()
    }

    pub fn new(file: &str) -> File {
        File {file: indexed_file::File::open_raw(file).unwrap()}
    }
}