extern crate fs_extra;

use std::borrow::BorrowMut;
use std::env::{current_dir, set_current_dir};
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::thread;
use crate::args::Args;
use file_manager::File;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::misc::{gen_names};
use crate::reader::BufReader;

pub mod args;
pub mod file_manager;
pub mod misc;
mod reader;


pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load(),
    };

    let file = File::new(&args.file, args.signed_file ).unwrap();
    let header = file.header();

    let (each, remain) = misc::lines_per_file(&file, args.number_of_files).unwrap_or_else(|| {
        eprintln!("Can't split {} lines between {} files", file.lines()-header, args.number_of_files);
        exit(1);
    });


    let names = Arc::new(Mutex::new( gen_names(&file, args.number_of_files)));
    let arc_file = Arc::new(Mutex::new( File::new(&args.file, args.signed_file ).unwrap()));
    let mut arc_buf = BufReader::open(file.file_path.as_str()).unwrap();

    let target_dir = format!("{}/{}", current_dir().unwrap().display(), &file.base_name().unwrap());
    fs_extra::dir::create(&target_dir, true).unwrap();

    set_current_dir(&target_dir).unwrap();

    for i in 1..=args.number_of_files {


            let r = misc::read_n_lines(&mut arc_buf, each);


            let mut f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .append(true)
                .open(&format!("{}/{}_{}.csv", current_dir().unwrap().display(), file.base_name().unwrap() ,i)).unwrap();


            f.write(r.as_bytes()).unwrap();
            println!("Wrote to {}", &format!("{}/{}_{}.csv", current_dir().unwrap().display(), file.base_name().unwrap() ,i) );

    }



}