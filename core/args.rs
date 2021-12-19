use clap::{ColorChoice, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, name = "spcsv", color(ColorChoice::Never))]
pub struct Args {
    /// The csv file to split
    #[clap(required = true)]
    pub(crate) file: String,

    /// The number of files to be created with the contents of the original csv file
    #[clap(required = true)]
    pub(crate) number_of_files: usize,

    /// The first line of FILE is NOT a header line. [By default it is]
    #[clap(short, long)]
    pub(crate) not_signed_file: bool,

    /// Write remaining lines in the last file [By default remaining rows are written to a new extra file]
    #[clap(short, long)]
    pub(crate) remaining_in_last: bool,

    /// Print when file is created
    #[clap(short, long)]
    pub(crate) verbose: bool,
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
