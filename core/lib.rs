use std::ops::{Deref, DerefMut};
use std::thread;
use crate::args::Args;
use file_manager::File;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub mod args;
pub mod file_manager;
pub mod misc;



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

    let mut handlers : Vec<JoinHandle<_>> = Vec::new();

    let arc_file = Arc::new(Mutex::new(file));

    for i in 1..=args.number_of_files {
        let s_file = arc_file.clone();
        handlers.push(thread::Builder::new().name(format!("thread{}",1)).spawn( move || {
            let r = misc::read_line_range(s_file.lock().unwrap().deref().clone(), i*each-each+header..i*each+header);
            println!("i: {} -- {:?}", i, r);
        }).unwrap())
    }

    for thread in handlers {
        thread.join().unwrap();
    }

}