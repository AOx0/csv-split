# split-csv

Simple command to split into n files the contents of one big csv.



## Installation

Simply run:

```
cargo install split-csv
```



## Usage

```
spcsv 0.0.4
Split a lage csv file into multiple files

USAGE:
    spcsv [OPTIONS] <FILE> <NUMBER_OF_FILES>

ARGS:
    <FILE>               The csv file to split
    <NUMBER_OF_FILES>    The number of files to be created with the contents of the original
                         file

OPTIONS:
    -h, --help                     Print help information
    -n, --not-signed-file          Flag to indicate the first line of FILE is a header line
    -r, --remaining-in-new-file    Flag to write remaining lines at an extra file (NUMBER_OF_FILES +
                                   1). When disabled writes remaining rows to the last file
                                   (NUMBER_OF_FILES)
    -v, --verbose                  Print when file is created
    -V, --version                  Print version information
```



Example:

```
spcsv COVID19.csv 100
```



The example above will split the lines of `COVID19.csv` along a hundred files with the names: `COVID19_1.csv`, `COVID19_2.csv`, `COVID19_3.csv`, `...`



