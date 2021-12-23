extern crate fs_extra;


use crate::args::Args;
use crate::reader::BufReader;
use file_manager::File;
use std::env::current_dir;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc};
use std::thread;
use std::thread::JoinHandle;
use sinner::Sin;

pub mod args;
pub mod file_manager;
pub mod misc;
mod reader;

#[derive(Clone)]
struct Shared {
    taget_dir: Arc<String>,
    headers: Arc<String>,
    file: Arc<File>
}

pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load(),
    };

    let file = File::new(&args.file, !args.not_signed_file).unwrap();
    let header = file.header();

    let (each, remain) = misc::lines_per_file(&file, args.number_of_files).unwrap_or_else(|| {
        eprintln!(
            "Can't split {} lines between {} files",
            file.lines() - header,
            args.number_of_files
        );
        exit(1);
    });

    let mut buf_reader = BufReader::open(file.file_path.as_str()).unwrap();

    let target_dir = format!(
        "{}/{}",
        current_dir().unwrap().display(),
        &file.base_name().unwrap()
    );
    fs_extra::dir::create(&target_dir, true).unwrap();

    let headers = misc::read_n_lines(&mut buf_reader, file.header());

    let mut handlers : Vec<JoinHandle<_>> = Vec::new();

    let shared = Sin::new(Shared {
        taget_dir: Arc::new(target_dir),
        headers:  Arc::new(headers),
        file:  Arc::new(file)
    });

    for i in 1..=args.number_of_files {
        let r =  misc::read_n_lines(&mut buf_reader, each);
        let op =  if i == args.number_of_files && remain != 0 {
            Some(misc::read_n_lines(&mut buf_reader, remain))
        } else { None };

        handlers.push(thread::Builder::new().name(format!("thread{}",1)).spawn( move || {
            let target_dir = &shared.taget_dir.clone();
            let name = shared.file.base_name().unwrap().clone();
            let headers = shared.headers.clone();
            let headers = headers.as_bytes();

            let mut f = std::fs::File::create( & format!(
                        "{}/{}_{}.csv",
                        target_dir,
                        name,
                        i
                    )).expect("Unable to create file");

            f.write_all(headers)
                .expect("Unable to write data");
            f.write_all(r.as_bytes()).expect("Unable to write data");
            if args.verbose {
                println!(
                    "Wrote {} rows to {}", each, &format!("{}/{}_{}.csv", target_dir, name, i)
                );
            }

            if i == args.number_of_files && remain > 0 {
                if args.remaining_in_last{
                    f.write_all(op.unwrap().as_bytes()).expect("Unable to write data");
                    if args.verbose {
                        println!(
                            "Wrote {} remaining rows to {}", remain,
                            &format!(
                                "{}/{}_{}.csv",
                                target_dir,
                                name,
                                i
                            )
                        );
                    }
                } else {
                    let mut f = std::fs::File::create(&format!(
                        "{}/{}_{}.csv",
                        target_dir,
                        name,
                        args.number_of_files + 1
                    ))
                        .expect("Unable to create file");

                    f.write_all(headers)
                        .expect("Unable to write data");
                    f.write_all(op.unwrap().as_bytes()).expect("Unable to write data");
                    if args.verbose {
                        println!(
                            "Wrote {} remaining rows to {}", remain,
                            &format!(
                                "{}/{}_{}.csv",
                                target_dir,
                                name,
                                args.number_of_files + 1
                            )
                        );
                    }
                }
            }
        }).unwrap());
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}
