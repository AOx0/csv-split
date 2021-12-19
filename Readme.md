# split-csv

Simple command to split into n files the contents of one big csv.



## Installation

Simply run:

```
cargo install split-csv
```

Or use the GitHub version:
```
cargo install --git https://github.com/AOx0/csv-split
```

## Usage

```HELP
spcsv 0.0.8
Split a lage csv file into multiple files

USAGE:
    spcsv [OPTIONS] <FILE> <NUMBER_OF_FILES>

ARGS:
    <FILE>               The csv file to split
    <NUMBER_OF_FILES>    The number of files to be created with the contents of the original csv
                         file

OPTIONS:
    -h, --help                 Print help information
    -n, --not-signed-file      The first line of FILE is NOT a header line. [By default it is]
    -r, --remaining-in-last    Write remaining lines in the last file [By default remaining rows are
                               written to a new extra file]
    -v, --verbose              Print when file is created
    -V, --version              Print version information
```



Example:

```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...`