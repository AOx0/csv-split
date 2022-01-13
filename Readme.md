# multi split-csv

Experimental branch

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
spcsv 0.1.0
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

The example above will split the lines of `Covid.csv` along a hundred files with the names: `Covid_1.csv`, `Covid_2.csv`, `Covid_3.csv`, `...`.



## Benchmarks

Comparison of benchmarking results for `spcsv v0.0.8` and `spcsv v0.1.0` splitting 12,133,531 rows from a CSV if 1.8G of size into 10 files:
```
spcsv Covid.csv 10
```
![][image-1]

Splitting the CSV into 50 files:
```
spcsv Covid.csv 50
```
![][image-2]

Splitting the CSV into 100 files:
```
spcsv Covid.csv 100
```
![][image-3]

Splitting the CSV into 10000 files:
```
spcsv Covid.csv 10000
```
![][image-4]


[image-1]:	https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/10files.png
[image-2]:	https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/50files.png
[image-3]:	https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/100files.png
[image-4]:	https://raw.githubusercontent.com/AOx0/csv-split/multithread/misc/10000files.png