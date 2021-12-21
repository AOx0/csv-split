extern crate fs_extra;

use std::borrow::BorrowMut;
use crate::args::Args;
use crate::reader::BufReader;
use file_manager::File;
use std::env::current_dir;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use sinner::Sin;

pub mod args;
pub mod file_manager;
pub mod misc;
mod reader;

#[derive(Clone)]
struct Shared {
    taget_dir: Arc<Mutex<String>>,
    headers: Arc<Mutex<String>>,
    reader: Arc<Mutex<BufReader>>,
    file: Arc<Mutex<File>>
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

    let buf_reader = Arc::new(Mutex::new(buf_reader));

    let shared = Sin::new(Shared {
        taget_dir: Arc::new(Mutex::new(target_dir)),
        headers:  Arc::new(Mutex::new(headers)),
        reader: buf_reader.clone(),
        file:  Arc::new(Mutex::new(file))
    });

    for i in 1..=args.number_of_files {
        handlers.push(thread::Builder::new().name(format!("thread{}",1)).spawn( move || {
            let r = misc::read_n_lines(shared.reader.lock().unwrap().borrow_mut(), each);
            let mut f = std::fs::File::create( & format!(
                        "{}/{}_{}.csv",
                        &shared.taget_dir.lock().unwrap(),
                        shared.file.lock().unwrap().base_name().unwrap(),
                        i
                    )).expect("Unable to create file");

            f.write_all(shared.headers.lock().unwrap().as_bytes())
                .expect("Unable to write data");
            f.write_all(r.as_bytes()).expect("Unable to write data");
            if args.verbose {
                println!(
                    "Wrote {} rows to {}", each, &format!("{}/{}_{}.csv", &shared.taget_dir.lock().unwrap(), shared.file.lock().unwrap().base_name().unwrap(), i)
                );
            }

            if i == args.number_of_files && remain > 0 {
                if args.remaining_in_last{
                    let r = misc::read_n_lines(shared.reader.lock().unwrap().borrow_mut(), remain);

                    f.write_all(r.as_bytes()).expect("Unable to write data");
                    if args.verbose {
                        println!(
                            "Wrote {} remaining rows to {}", remain,
                            &format!(
                                "{}/{}_{}.csv",
                                &shared.taget_dir.lock().unwrap(),
                                shared.file.lock().unwrap().base_name().unwrap(),
                                i
                            )
                        );
                    }
                } else {
                    let r = misc::read_n_lines(shared.reader.lock().unwrap().borrow_mut(), remain);
                    let mut f = std::fs::File::create(&format!(
                        "{}/{}_{}.csv",
                        &shared.taget_dir.lock().unwrap(),
                        shared.file.lock().unwrap().base_name().unwrap(),
                        args.number_of_files + 1
                    ))
                        .expect("Unable to create file");

                    f.write_all(shared.headers.lock().unwrap().as_bytes())
                        .expect("Unable to write data");
                    f.write_all(r.as_bytes()).expect("Unable to write data");
                    if args.verbose {
                        println!(
                            "Wrote {} remaining rows to {}", remain,
                            &format!(
                                "{}/{}_{}.csv",
                                &shared.taget_dir.lock().unwrap(),
                                shared.file.lock().unwrap().base_name().unwrap(),
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
