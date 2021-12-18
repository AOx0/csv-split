extern crate fs_extra;

use std::env::{current_dir};
use std::io::Write;
use crate::args::Args;
use file_manager::File;
use std::process::exit;
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


    let mut arc_buf = BufReader::open(file.file_path.as_str()).unwrap();

    let target_dir = format!("{}/{}", current_dir().unwrap().display(), &file.base_name().unwrap());
    fs_extra::dir::create(&target_dir, true).unwrap();

    for i in 1..=args.number_of_files {
            let r = misc::read_n_lines(&mut arc_buf, each);
            let mut f = std::fs::File::create(&format!("{}/{}_{}.csv", &target_dir, file.base_name().unwrap() ,i)).expect("Unable to create file");

            f.write_all(r.as_bytes()).expect("Unable to write data");
            println!("Wrote to {}", &format!("{}/{}_{}.csv", &target_dir, file.base_name().unwrap() ,i) );

    }

    let r = misc::read_n_lines(&mut arc_buf, remain);
    let mut f = std::fs::File::create(&format!("{}/{}_{}.csv", &target_dir, file.base_name().unwrap() ,args.number_of_files+1)).expect("Unable to create file");

    f.write_all(r.as_bytes()).expect("Unable to write data");
    println!("Wrote to {}", &format!("{}/{}_{}.csv", &target_dir, file.base_name().unwrap() ,args.number_of_files+1) );
}