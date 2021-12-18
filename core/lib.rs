extern crate fs_extra;

use std::env::{current_dir, set_current_dir};
use std::ops::{Deref};
use std::thread;
use crate::args::Args;
use file_manager::File;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::misc::{gen_names, write_contents_to};

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

    let names = Arc::new(Mutex::new( gen_names(&file, args.number_of_files)));
    let arc_file = Arc::new(Mutex::new( File::new(&args.file, args.signed_file ).unwrap()));

    let target_dir = format!("{}/{}", current_dir().unwrap().display(), &file.base_name().unwrap());
    fs_extra::dir::create(&target_dir, true).unwrap();

    set_current_dir(&target_dir).unwrap();

    println!("{:?}", current_dir().unwrap().display());

    for i in 1..=args.number_of_files {
        let s_file = arc_file.clone();
        handlers.push(thread::Builder::new().name(format!("thread{}",i)).spawn( move || {
            let file = s_file.lock().unwrap().deref().clone();
            let r = misc::read_line_range(file.clone(), i*each-each+header..i*each+header);
            println!("i: {} -- {:?}", i, r);

            for line in r {
                println!("{}", &format!("{}/{}_{}.csv", current_dir().unwrap().display(), file.base_name().unwrap() ,i));
                write_contents_to(&format!("{}/{}_{}.csv", current_dir().unwrap().display(), file.base_name().unwrap() ,i),  line.as_bytes()).unwrap();
            }
        }).unwrap())
    }

    for thread in handlers {
        thread.join().unwrap();
    }

}