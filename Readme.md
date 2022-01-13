# split-csv

Experimental branch

Simple command to split into *N* number of files the contents of one big CSV.

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
spcsv 0.1.2
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

The example above will split the lines of `Covid.csv` along a hundred files with the names: `Covid_1.csv`, `Covid_2.csv`, `Covid_3.csv`, `...`, `Covid_99.csv`, `Covid_100.csv`.