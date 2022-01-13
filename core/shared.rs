use crate::{b_file, file_manager, BufReader};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct IShared {
    pub paths: Vec<String>,
    pub headers: String,
}

pub type MShared = Arc<Mutex<BufReader>>;

pub trait Crate {
    fn n(value: BufReader) -> Self;
    fn get_headers(&self, file: &file_manager::File) -> String;
}

impl Crate for MShared {
    fn n(value: BufReader) -> Self {
        Arc::new(Mutex::new(value))
    }

    fn get_headers(&self, file: &file_manager::File) -> String {
        crate::misc::read_n_lines(b_file!(self), file.header())
    }
}
