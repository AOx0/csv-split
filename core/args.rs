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
}

impl Args {
    pub fn load() -> Args {
        Args::parse()
    }
}
