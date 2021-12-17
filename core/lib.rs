use crate::args::Args;

mod args;
mod file_manager;

pub fn app(args: Option<Args>) {
    let args: Args = match args {
        Some(a) => a,
        None => Args::load()
    };

    println!("{:?}", args);
}