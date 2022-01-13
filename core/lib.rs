use futures::future::join_all;
use std::env::current_dir;
use std::process::exit;
use file_manager::File;
use std::sync::Arc;
use std::io::Write;

pub mod reader;
pub mod shared;
pub mod file_manager;
pub mod args;
pub mod misc;

use crate::reader::BufReader;
use crate::args::Args;
use crate::misc::*;
use shared::*;

#[tokio::main]
pub async fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load(),
    };

    let extra = if args.remaining_in_last { 0 } else { 1 };
    let file: File = File::new(&args.file, !args.not_signed_file).unwrap();
    let buf_reader: MShared = MShared::n(BufReader::open(file.file_path.as_str()).unwrap());

    let (each, remain) = lines_per_file(&file, args.number_of_files).unwrap_or_else(|| {
        eprintln!(
            "Can't split {} lines within {} files",
            file.lines() - file.header(),
            args.number_of_files
        );
        exit(1);
    });

    fs_extra::dir::create(get_target_directory(&file), true).unwrap();

    let headers = buf_reader.get_headers(&file);
    let paths = gen_paths(&file, extra + args.number_of_files);
    let shared = Arc::new(IShared { paths, headers });

    let mut handlers = vec![];
    for i in 0..=args.number_of_files - extra {
        let buf_reader_clone = Arc::clone(&buf_reader);
        let shared = Arc::clone(&shared);

        handlers.push(tokio::task::spawn(async move {
            let text;
            let remaining_text;
            {
                let mut reader = &mut *buf_reader_clone.lock().unwrap();
                text = read_n_lines(&mut reader, each);
                remaining_text = if i == args.number_of_files - extra && remain > 0 {
                    Some(read_n_lines(&mut reader, remain))
                } else {
                    None
                };
            }

            let mut f = std::fs::File::create(&shared.paths[i]).expect("Unable to create file");

            f.write_all(shared.headers.as_bytes())
                .expect("Unable to write data");
            f.write_all(text.as_bytes()).expect("Unable to write data");
            if args.verbose {
                println!("Wrote {} rows to {}", each, &shared.paths[i]);
            }

            if i == args.number_of_files - extra && remain > 0 {
                if args.remaining_in_last {
                    f.write_all(remaining_text.unwrap().as_bytes())
                        .expect("Unable to write data");
                    if args.verbose {
                        println!("Wrote {} remaining rows to {}", remain, &shared.paths[i]);
                    }
                } else {
                    let mut f =
                        std::fs::File::create(&shared.paths[i + 1])
                            .expect("Unable to create file");

                    f.write_all(shared.headers.as_bytes())
                        .expect("Unable to write data");
                    f.write_all(remaining_text.unwrap().as_bytes())
                        .expect("Unable to write data");
                    if args.verbose {
                        println!(
                            "Wrote {} remaining rows to {}",
                            remain,
                            &shared.paths[i + 1]
                        );
                    }
                }
            }
        }));
    }

    join_all(handlers).await;
}
