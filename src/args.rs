use clap::{ColorChoice, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, name="spcsv", author, color(ColorChoice::Never))]
pub struct Args {
    /// The csv file to split
    #[clap(required = true)]
    file: String,

    /// The number of files to be created with the contents of the original file
    #[clap(required = true)]
    number_of_files: i32,
}

impl Args { pub fn load() -> Args { Args::parse() } }