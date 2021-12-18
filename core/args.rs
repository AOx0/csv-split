use clap::{ColorChoice, Parser};

#[derive(Parser, Debug)]
#[clap(about, version, name = "spcsv", color(ColorChoice::Never))]
pub struct Args {
    /// The csv file to split
    #[clap(required = true)]
    pub(crate) file: String,

    /// The number of files to be created with the contents of the original file
    #[clap(required = true)]
    pub(crate) number_of_files: usize,

    /// Flag to indicate the first line of FILE is a header line
    #[clap(short, long)]
    pub(crate) signed_file: bool,

    /// Flag to write remaining lines at an extra file (NUMBER_OF_FILES + 1).
    /// When disabled writes remaining rows to the last file (NUMBER_OF_FILES)
    #[clap(short, long)]
    pub(crate) remaining_in_new_file: bool,
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
